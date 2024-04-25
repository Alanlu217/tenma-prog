#[derive(Debug)]
pub struct Config {
    port: String,
    file_path: String,
}

impl Config {
    pub fn from_args(args: &Vec<String>) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(format!("Expected two arguments, recieved {}", args.len() - 1));
        }

        Ok(Self { port: args[1].clone(), file_path: args[2].clone() })
    }
}
