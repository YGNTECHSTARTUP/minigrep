use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };
    match result {
        Ok(lines) => {
            for line in lines {
                println!("{line}")
            }
        }
        Err(err) => {
            println!("{err}");
            return Err(err).into();
        }
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not Enough Arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search_case_sensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut v = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            v.push(line.trim());
        }
    }
    if v.is_empty() {
        return Err("NOT FOUND".into());
    }
    Ok(v)
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut v = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            v.push(line.trim());
        }
    }
    if v.is_empty() {
        return Err("NOT FOUND".into());
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "Pick";
        let contents = "\n
        Rust:\n
        safe,fast,productive.\n
        Pick Three.";
        assert_eq!(
            vec!["Pick Three."],
            search_case_sensitive(query, contents).unwrap()
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "pick";
        let contents = "\n
        Rust:\n
        safe,fast,productive.\n
        Pick Three.";
        assert_eq!(
            vec!["Pick Three."],
            search_case_insensitive(query, contents).unwrap()
        );
    }
}
