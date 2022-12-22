#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub port: String,
    pub addr: String,
    pub max_threads: Option<usize>,
    pub default_file: String,
    pub err404_file: Option<String>,
}

impl Config {

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
            other => {
                let msg = format!("No such option: {other}");
                return Err(msg);
            },
        }
        Ok(())
    }

    pub fn build_from_cmdline(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        let mut config = Config {
            port: String::from("127.0.0.1"),
            addr: String::from("6969"),
            max_threads: None,
            default_file: String::from("index.html"),
            err404_file: None,
        };

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
        });
    }
}
