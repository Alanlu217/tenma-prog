#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: String,
    pub file_path: String,
}

impl Config {
    pub fn from_args(args: &Vec<String>) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(format!("Expected two arguments, recieved {}", args.len() - 1));
        }

        Ok(Self { port: args[1].clone(), file_path: args[2].clone() })
    }
}

#[test]
fn test_config_from_args() {
    let args: Vec<String> = vec!["file_path".to_string(), "test1".to_string(), "test2".to_string()];

    assert_eq!(
        Config::from_args(&args),
        Ok(Config { port: "test1".to_string(), file_path: "test2".to_string() })
    );
}
