use std::{env, path::Path};

enum Detection {
    Rust,
    Node,
    Both,
    Unsupported,
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
        Detection::Rust => println!("Rust project detected"),
        Detection::Node => println!("Node project detected"),
        Detection::Both => println!("Both detected"),
        Detection::Unsupported => println!("Unsupported directory"),
    }

    Ok(())
}
