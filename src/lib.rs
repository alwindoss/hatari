use clap::Parser;

mod repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the repository
    #[arg(short, long)]
    repository: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub fn run() -> Result<bool, Box<dyn std::error::Error>> {
    repository::repository::create("loc");
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Repository to be used: {}", args.repository);
    }
    return Ok(true);
}
