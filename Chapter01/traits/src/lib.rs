use std::io::{Read, Write};
// Structs

///
/// Configuration for our application
///
pub struct Config {
    values: Vec<(String, String)>,
}

///
/// A service for managing a configuration
///
pub struct KeyValueConfigService {}

// Traits

///
/// Provides a get() function to return values associated with
/// the specified key.
///
pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}

///
/// Write a config
///
pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}

///
/// Read a config
///
pub trait ConfigReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config>;
}

// Impls

impl Config {
    pub fn new(values: Vec<(String, String)>) -> Config {
        Config { values: values }
    }
}

impl KeyValueConfigService {
    pub fn new() -> KeyValueConfigService {
        KeyValueConfigService {}
    }
}

impl ConfigWriter for KeyValueConfigService {
    fn write(&self, config: Config, mut to: &mut impl Write) -> std::io::Result<()> {
        for v in config.values {
            writeln!(&mut to, "{0}={1}", v.0, v.1)?;
        }
        Ok(())
    }
}

impl ConfigReader for KeyValueConfigService {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer)?;

        // chain iterators together and collect the results
        let values: Vec<(String, String)> = buffer
            .split_terminator("\n") // split
            .map(|line| line.trim()) // remove whitespace
            .filter(|line| {
                // filter invalid lines
                let pos = line.find("=").unwrap_or(0);
                pos > 0 && pos < line.len() - 1
            })
            .map(|line| {
                // create a tuple from a line
                let parts = line.split("=").collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect(); // transform it into a vector
        Ok(Config::new(values))
    }
}

impl ValueGetter for Config {
    fn get(&self, s: &str) -> Option<String> {
        self.values.iter().find_map(|tuple| {
            if &tuple.0 == s {
                Some(tuple.1.clone())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn config_get_value() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);
        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("HELLO"), None);
    }

    #[test]
    fn keyvalueconfigservice_write_config() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);

        let service = KeyValueConfigService::new();
        let mut target = vec![];
        assert!(service.write(config, &mut target).is_ok());

        assert_eq!(
            String::from_utf8(target).unwrap(),
            "hello=world\n".to_string()
        );
    }

    #[test]
    fn keyvalueconfigservice_read_config() {
        let service = KeyValueConfigService::new();
        let readable = &format!("{}\n{}", "hello=world", "a=b").into_bytes();

        let config = service
            .read(&mut Cursor::new(readable))
            .expect("Couldn't read from the vector");

        assert_eq!(
            config.values,
            vec![
                ("hello".to_string(), "world".to_string()),
                ("a".to_string(), "b".to_string())
            ]
        );
    }
}
