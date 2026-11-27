use std::{env, path::Path, sync::LazyLock};

enum Detection {
    Rust,
    Node,
    Both,
    Unsupported,
}

struct Task {
    name: String,
    program: String,
    args: Vec<String>,
}

static CARGO_CMDS: LazyLock<Vec<Task>> = LazyLock::new(|| {
    vec![
        Task {
            name: "Build".to_string(),
            program: "cargo".to_string(),
            args: vec!["build".to_string()],
        },
        Task {
            name: "Run".to_string(),
            program: "cargo".to_string(),
            args: vec!["run".to_string()],
        },
        Task {
            name: "Test".to_string(),
            program: "cargo".to_string(),
            args: vec!["test".to_string()],
        },
        Task {
            name: "Check".to_string(),
            program: "cargo".to_string(),
            args: vec!["check".to_string()],
        },
    ]
});

fn print_tasks(tasks: &[Task]) {
    println!("Commands: ");
    for task in tasks {
        println!(" {}: {} {}", task.name, task.program, task.args.join(" "));
    }
}

fn manifest_is_file(path: &Path) -> std::io::Result<bool> {
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(metadata.is_file()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn detect_project(directory: &Path) -> std::io::Result<Detection> {
    let has_cargo = manifest_is_file(&directory.join("Cargo.toml"))?;
    let has_package = manifest_is_file(&directory.join("package.json"))?;

    Ok(match (has_cargo, has_package) {
        (true, false) => Detection::Rust,
        (false, true) => Detection::Node,
        (true, true) => Detection::Both,
        (false, false) => Detection::Unsupported,
    })
}

fn node_tasks(directory: &Path) -> std::io::Result<Vec<Task>> {
    let package_path = directory.join("package.json");
    let package_contents = std::fs::read_to_string(&package_path)?;

    let package: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&package_contents).map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid {}: {error}", package_path.display()),
            )
        })?;

    let scripts = match package.get("scripts") {
        None => return Ok(Vec::new()),
        Some(value) => value.as_object().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "package.json: 'scripts' must be an object",
            )
        })?,
    };

    let mut tasks = Vec::new();

    for (name, command) in scripts {
        if !command.is_string() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("package.json: script `{name} must contain a string"),
            ));
        }

        tasks.push(Task {
            name: name.clone(),
            program: "npm".to_string(),
            args: vec!["run".to_string(), "--".to_string(), name.clone()],
        });
    }

    Ok(tasks)
}

fn main() -> std::io::Result<()> {
    println!("LazyFlow: Project Detection");

    let directory = env::current_dir()?;
    let detection = detect_project(&directory)?;

    println!("Current Directory: {}", directory.display());

    match detection {
        Detection::Rust => {
            println!("Rust project detected");
            print_tasks(CARGO_CMDS.as_slice());
        }
        Detection::Node => {
            println!("Node.js project detected");
            let task = node_tasks(&directory)?;
            if task.is_empty() {
                print!("No npm scripts are defined.");
            } else {
                print_tasks(&task);
            }
        }
        Detection::Both => println!("Both detected"),
        Detection::Unsupported => println!("Unsupported directory"),
    }

    Ok(())
}
