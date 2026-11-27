use std::env;

fn main() -> std::io::Result<()> {
    println!("LazyFlow: Project Detection");

    let directory = env::current_dir()?;
    println!("Current Directory: {}", directory.display());

    let has_cargo = directory.join("Cargo.toml").is_file();
    println!("Cargo.toml found: {}", has_cargo);

    Ok(())
}
