use std::{env::args, path::PathBuf};

use clap::{Parser, Subcommand};
mod repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the program
    name: String,
    #[arg(short, long, default_value = "./repo")]
    /// Path where the repository is created
    repo: PathBuf,
}

pub fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("Name: {}", cli.name);
    println!("Repository will bre created @ {:?}", cli.repo);
    let res = repository::create(&cli.repo);
    match res {
        Ok(_) => println!("Created repository"),
        Err(_) => (),
    }
    return Ok(true);
}
