use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
struct Container {
    name: String,
    image: String,
    status: String,
}

impl Container {
    fn new(name: String, image: String, status: String) -> Self {
        Container {
            name,
            image,
            status,
        }
    }
}

enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn as_str(&self) -> &str {
        match self {
            Orientation::Horizontal => "h",
            Orientation::Vertical => "v",
        }
    }
}

fn main() {
    //I am the best
    //store each container info box in a vector
    let mut names = Vec::new();
    let mut images = Vec::new();
    let mut status = Vec::new();
    let mut containers: Vec<Container> = Vec::new();

    // Run the docker ps command and capture the output
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.Names}}|{{.Image}}|{{.Status}}"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                println!("(label :class \"docker-empty\" :text \"No containers running\")");
            } else {
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.splitn(3, '|').collect();
                    let cont = Container::new(
                        parts.first().unwrap_or(&"").to_string(),
                        parts.get(1).unwrap_or(&"").to_string(),
                        parts.get(2).unwrap_or(&"").to_string(),
                    );

                    create_box(&cont, "", &mut names);
                    create_box(&cont, "", &mut images);
                    create_box(&cont, "", &mut status);

                    containers.push(cont);
                }
            }
        }
        Err(_) => {
            println!("(label :class \"docker-empty\" :text \"No containers running\")");
        }
    }

    let name_section = create_section("name", Orientation::Vertical, names);
    let image_section = create_section("image", Orientation::Vertical, images);
    let status_section = create_section("status", Orientation::Vertical, status);

    let main_box = create_section(
        "main",
        Orientation::Horizontal,
        vec![name_section, image_section, status_section],
    );
    println!("{}", main_box);
}

fn create_section(section: &str, orinentation: Orientation, list: Vec<String>) -> String {
    let mut main_box = String::new();
    main_box.push_str(&format!("(box :orientation \"{}\" :class \"docker-{}-section\" :space-evenly \"false\" :spacing 4 \n", orinentation.as_str(), section));

    match section {
        "name" => main_box.push_str("  (label :class \"docker-text-name\" :text \"Container Name\" :halign \"center\" :valign \"center\")\n"),
        "image" => main_box.push_str("  (label :class \"docker-text-image\" :text \"Image\" :halign \"center\" :valign \"center\")\n"),
        "status" => main_box.push_str("  (label :class \"docker-text-status\" :text \"Status\" :halign \"center\" :valign \"center\")\n"),
        _ => {}
    }

    for box_str in &list {
        main_box.push_str(box_str.to_string().as_str());
    }
    main_box.push_str(")\n");

    main_box
}

fn create_box(container: &Container, icon: &str, list: &mut Vec<String>) {
    let mut cont = String::new();

    cont.push_str(&format!("(box :class \"docker-{}-box\" :orientation \"horizontal\" :space-evenly \"false\" :spacing 10 \n", container.name));

    cont.push_str(&format!("  (label :class \"docker-{}-icon\"    :text \"{}\"  :halign \"end\"  :valign \"center\" )\n", container.name, icon));
    match icon {
        "" => cont.push_str(&format!("  (label :class \"docker-text-name\"   :text \"{}\"  :halign \"start\" :valign \"center\")\n", container.name)),
        "" => cont.push_str(&format!("  (label :class \"docker-text-image\"  :text \"{}\"  :halign \"start\" :valign \"center\")\n", container.image)),
        "" => cont.push_str(&format!("  (label :class \"docker-text-status\" :text \"{}\"  :halign \"start\" :valign \"center\")\n", container.status)),
        _ => {}
    }
    cont.push(')');

    list.push(cont);
}
