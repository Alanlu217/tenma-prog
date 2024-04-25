use std::{ fmt::Display, fs };

pub struct TenmaScript {
    contents: Vec<String>,
}

impl TenmaScript {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let x = fs
            ::read_to_string(path)?
            .lines()
            .map(|x| x.to_string())
            .collect();

        Ok(TenmaScript { contents: x })
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        for line in self.contents.iter() {
            out.push_str(format!("{}\n", line).as_str());
        }

        out
    }
}

impl Display for TenmaScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
