#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub port: String,
    pub addr: String,
    pub max_threads: Option<usize>,
    pub default_file: String,
    pub err404_file: Option<String>,
    pub limit_requests: Option<usize>,
}

impl Config {

    /// Creates a new default config.
    pub fn new() -> Config {
        Config {
            port: String::from("6969"),
            addr: String::from("127.0.0.1"),
            max_threads: None,
            default_file: String::from("index.html"),
            err404_file: None,
            limit_requests: None,
        }
    }

    fn set_option(&mut self, arg: String, value: String) -> Result<(), String> {
        match &arg[..] {
            "--port"         | "-p" => self.port         = value.to_string(),
            "--addr"         | "-a" => self.addr         = value.to_string(),
            "--default-file" | "-f" => self.default_file = value.to_string(),
            "--err404-file"  | "-e" => self.err404_file  = Some(value.to_string()),
            "--max-threads"  | "-t" => {
                match value.to_string().parse::<usize>() {
                    Err(_) => {
                        let msg = format!("{value} is not a valid number!");
                        return Err(msg);
                    },
                    Ok(n) if n <= 0 => {
                        let msg = format!("{value} is not integer greater then 0!");
                        return Err(msg);
                    },
                    Ok(n) => self.max_threads = Some(n),
                };
            },
            "--limit-requests" | "-l" => {
                match value.to_string().parse::<usize>() {
                    Err(_) => {
                        let msg = format!("{value} is not a valid number!");
                        return Err(msg);
                    },
                    Ok(n) => self.limit_requests = Some(n),
                }
            }
            other => {
                let msg = format!("No such option: {other}");
                return Err(msg);
            },
        }
        Ok(())
    }

    /// Builds a new config from a `Iterator<Item = String>`, usually the `std::env::args`. Returns
    /// `Err(String)` with a message if config could not be parsed.
    ///
    /// # Examples
    /// ```
    /// use std::{process, env};
    /// use qst::config;
    /// let args = env::args();
    /// let config = config::Config::build_from_cmdline(args).unwrap_or_else(|msg| {
    ///     eprintln!("Error parsing config: {msg}");
    ///     process::exit(1);
    /// });
    /// ```
    pub fn build_from_cmdline(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        let mut config = Config::new();

        if args.next().is_none() {
            return Ok(config);
        }

        loop {
            let arg = match args.next() {
                Some(arg) => arg,
                None => return Ok(config),
            };
            let value = match args.next() {
                Some(value) => value,
                None => {
                    let msg = format!("No value specified for {arg}");
                    return Err(msg);
                },
            };
            config.set_option(arg, value)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_sets_options() {
        let vec_args = vec![
            String::from("qst"),
            String::from("--port"),
            String::from("420"),
            String::from("--addr"),
            String::from("192.168.0.1"),
            String::from("--max-threads"),
            String::from("8"),
            String::from("--default-file"),
            String::from("home.html"),
            String::from("--err404-file"),
            String::from("404.html"),
            String::from("--limit-requests"),
            String::from("4"),
        ];
        let args = vec_args.iter().map(|s| s.to_string());
        let config = match Config::build_from_cmdline(args) {
            Ok(config) => config,
            Err(msg)   => panic!("Tried valid config, got {msg} instead"),
        };
        assert_eq!(config, Config {
            port: String::from("420"),
            addr: String::from("192.168.0.1"),
            max_threads: Some(8),
            default_file: String::from("home.html"),
            err404_file: Some(String::from("404.html")),
            limit_requests: Some(4),
        });
    }

    #[test]
    fn config_sets_default_with_empty() {
        let vec_args: Vec<String> = vec![];
        let args = vec_args.iter().map(|s| s.to_string());
        let config = match Config::build_from_cmdline(args) {
            Ok(config) => config,
            Err(msg) => panic!("Tried valid empty config, got {msg} instead"),
        };
        assert_eq!(config, Config::new());
    }

    #[test]
    fn config_sets_default_with_one_arg() {
        let vec_args = vec![String::from("prog_name")];
        let args = vec_args.iter().map(|s| s.to_string());
        let config = match Config::build_from_cmdline(args) {
            Ok(config) => config,
            Err(msg) => panic!("Tried valid empty config, got {msg} instead"),
        };
        assert_eq!(config, Config::new());
    }
}
