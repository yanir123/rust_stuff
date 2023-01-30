use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file: String,
    case_sensitive: bool,
}
impl Config {
    pub fn build(argv: &[String]) -> Result<Config, &'static str> {
        if argv.len() < 3 {
            return Err("Usage is: <query> <file-path>");
        }

        let query = argv[1].clone();
        let file = argv[2].clone();
        let case_sensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.file)?;

    let res = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_isensitive(&config.query, &content)
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_isensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = vec![];

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            res.push(line);
        }
    }

    res
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}
