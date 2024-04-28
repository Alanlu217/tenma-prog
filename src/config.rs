#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub ports: Vec<Option<String>>,
}

impl Config {
    pub fn from_args(args: &Vec<String>) -> Result<Self, String> {
        if !(3..).contains(&args.len()) {
            return Err(format!(
                "Expected two or more arguments, recieved {}\n\nArguments should be in format: lua_file_path serial_port...\nor: lua_file_path",
                args.len() - 1
            ));
        }

        let mut v = vec![];
        for i in 2..args.len() {
            if args[i] == "tester" {
                v.push(None)
            } else {
                v.push(Some(format!("{}", args[i])));
            }
        }
        Ok(Self {
            file_path: args[1].clone(),
            ports: v,
        })
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
            ports: vec![Some("test1".to_string())],
            file_path: "test2".to_string()
        })
    );
}
