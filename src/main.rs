use std::{env, path::Path};

enum Detection {
    Rust,
    Node,
    Both,
    Unsupported,
}

fn detect_project(directory: &Path) -> Detection {
    let has_cargo = directory.join("Cargo.toml").is_file();
    let has_package = directory.join("package.json").is_file();

    match (has_cargo, has_package) {
        (true, false) => Detection::Rust,
        (false, true) => Detection::Node,
        (true, true) => Detection::Both,
        (false, false) => Detection::Unsupported,
    }
}

fn main() -> std::io::Result<()> {
    println!("LazyFlow: Project Detection");

    let directory = env::current_dir()?;
    let detection = detect_project(&directory);
    match detection {
        Detection::Rust => println!("Rust project detected"),
        Detection::Node => println!("Node project detected"),
        Detection::Both => println!("Both detected"),
        Detection::Unsupported => println!("Unsupported directory"),
    }
    println!("Current Directory: {}", directory.display());

    Ok(())
}
