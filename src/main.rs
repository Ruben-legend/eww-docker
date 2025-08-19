use std::process::Command;

fn main() {
    //store each container info box in a vector
    let mut names = Vec::new();
    let mut images = Vec::new();
    let mut status = Vec::new();

    // Run the docker ps command and capture the output
    let output = Command::new("docker")
        .args(&["ps", "--format", "{{.Names}}|{{.Image}}|{{.Status}}"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                println!("(label :class \"docker-empty\" :text \"No containers running\")");
            } else {
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.splitn(3, '|').collect();
                    let name = parts.get(0).unwrap_or(&"");
                    let image = parts.get(1).unwrap_or(&"");
                    let state = parts.get(2).unwrap_or(&"");

                    create_box(name, "", &mut names);
                    create_box(image, "", &mut images);
                    create_box(state, "", &mut status);
                }
            }
        }
        Err(_) => {
            println!("(label :class \"docker-empty\" :text \"No containers running\")");
        }
    }

    let name_section = create_section("name", "v", names);
    let image_section = create_section("image", "v", images);
    let status_section = create_section("status", "v", status);

    let main_box = create_section(
        "main",
        "h",
        vec![name_section, image_section, status_section],
    );
    println!("{}", main_box);
}

fn create_section(name: &str, orinentation: &str, list: Vec<String>) -> String {
    let mut main_box = String::new();
    main_box.push_str(&format!("(box :orientation \"{}\" :class \"docker-{}-section\" :space-evenly \"false\" :spacing 4 \n", orinentation, name));

    match name {
        "name" => main_box.push_str("  (label :class \"docker-text-name\" :text \"Container Name\" :halign \"center\" :valign \"center\")\n"),
        "image" => main_box.push_str("  (label :class \"docker-text-image\" :text \"Image\" :halign \"center\" :valign \"center\")\n"),
        "status" => main_box.push_str("  (label :class \"docker-text-status\" :text \"Status\" :halign \"center\" :valign \"center\")\n"),
        _ => {}
    }

    for box_str in &list {
        main_box.push_str(&format!("{}", box_str));
    }
    main_box.push_str(")\n");

    main_box
}

fn create_box(name: &str, icon: &str, list: &mut Vec<String>) {
    let mut container = String::new();

    container.push_str(&format!("(box :class \"docker-{}-box\" :orientation \"horizontal\" :space-evenly \"false\" :spacing 10 \n", name));

    container.push_str(&format!("  (label :class \"docker-{}-icon\"    :text \"{}\"  :halign \"end\"  :valign \"center\" )\n", name, icon));
    container.push_str(&format!("  (label :class \"docker-{}-text\"    :text \"{}\" :halign \"start\"  :valign \"center\" )\n", name, name));

    container.push_str(")");

    list.push(container);
}
