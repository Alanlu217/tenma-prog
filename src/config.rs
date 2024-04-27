#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub port: Option<String>,
}

impl Config {
    pub fn from_args(args: &Vec<String>) -> Result<Self, String> {
        if !(2..=3).contains(&args.len()) {
            return Err(format!(
                "Expected one or two arguments, recieved {}\n\nArguments should be in format: lua_file_path serial_port\nor: lua_file_path",
                args.len() - 1
            ));
        }

        match args.len() {
            2 => Ok(Self {
                file_path: args[1].clone(),
                port: None,
            }),
            3 => Ok(Self {
                file_path: args[1].clone(),
                port: Some(args[2].clone()),
            }),
            _ => {
                return Err(format!(
                "Expected one or two arguments, recieved {}\n\nArguments should be in format: lua_file_path serial_port\nor: lua_file_path",
                args.len() - 1
            ));
            }
        }
    }
}

#[test]
fn test_config_from_args() {
    let args: Vec<String> = vec![
        "file_path".to_string(),
        "test1".to_string(),
        "test2".to_string(),
    ];

    assert_eq!(
        Config::from_args(&args),
        Ok(Config {
            port: Some("test1".to_string()),
            file_path: "test2".to_string()
        })
    );
}
