use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut ignore_case = false;

        let mut filtered_args = args.filter(|arg| match arg.as_str() {
            "-i" => {
                ignore_case = true;
                false
            }
            _ => true,
        });

        filtered_args.next(); // We don't need the program name

        let query = match filtered_args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match filtered_args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        ignore_case = ignore_case || env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn config_error_too_few_arguments() {
        let result = Config::build([String::from("foo")].into_iter());
        match result {
            Ok(_) => panic!("This should fail"),
            Err(e) => assert!(e.to_lowercase().contains("query")),
        }
    }

    #[test]
    fn config_three_arguments() {
        let result = Config::build(
            [
                String::from("minigrep"),
                String::from("to"),
                String::from("poem.txt"),
            ]
            .into_iter(),
        )
        .unwrap();

        assert_eq!(result.query, "to");
        assert_eq!(result.file_path, "poem.txt");
    }

    #[test]
    fn config_can_set_case_insensitive() {
        let result = Config::build(
            [
                String::from("minigrep"),
                String::from("-i"),
                String::from("to"),
                String::from("poem.txt"),
            ]
            .into_iter(),
        )
        .unwrap();

        assert!(result.ignore_case)
    }

    // In the real world, test for and handle args.len() > 3 && args[1] != "-i"
}
