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
        Detection::Node => println!("Node project detected"),
        Detection::Both => println!("Both detected"),
        Detection::Unsupported => println!("Unsupported directory"),
    }

    Ok(())
}
