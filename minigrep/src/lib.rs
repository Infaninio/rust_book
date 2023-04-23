use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("Searching for '{}' in {}", config.query, config.file_path);

    let file_contents = fs::read_to_string(config.file_path)?;

    println!("With text \n{file_contents}");
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
}
impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Result::Err("Not enough arguments provided!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Result::Ok(Config {query, file_path})
    }
}