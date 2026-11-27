use std::env;

fn main() -> std::io::Result<()> {
    println!("LazyFlow: Project Detection");

    let directory = env::current_dir()?;
    println!("Current Directory: {}", directory.display());

    let has_cargo = directory.join("Cargo.toml").is_file();
    let has_package = directory.join("package.json").is_file();

    match (has_cargo, has_package) {
        (true, false) => println!("Rust project detected"),
        (false, true) => println!("Node.js project detected"),
        (true, true) => println!("Both detected; explicit selection will be needed"),
        (false, false) => println!("Unsupported directory"),
    }

    Ok(())
}
