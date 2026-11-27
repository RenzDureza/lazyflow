use std::env;

fn main() -> std::io::Result<()> {
    println!("LazyFlow: Project Detection");

    let directory = env::current_dir()?;
    println!("Current Directory: {}", directory.display());
    Ok(())
}
