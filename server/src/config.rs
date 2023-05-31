use crate::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

const ENV_PORT: &str = "PORT";
const DEFAULT_PORT: u16 = 8080;

impl Config {
    pub fn load() -> Result<Config, Error> {
        dotenv::dotenv().ok();

        let port = std::env::var(ENV_PORT)
            .ok()
            .map_or(Ok(DEFAULT_PORT), |env_val| env_val.parse())?;

        let database_url = "postgres://h3cker:postgres@localhost:5432/zkrat".to_string();
        Ok(Config { port, database_url })
    }
}


/*
#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_load_config() {
        let config = Config::load().unwrap_or({
            println!("Failed to load config, using default values");
            Config {
                port: DEFAULT_PORT,
                database_url: "postgres://h3cker:postgres@localhost:5432/h3cker"
                .to_string(),
            }
        });
        
        assert_eq!(config.port, 8080);
        dbg!(&config);
        //  assert_eq!(
            //      config.database_url,
            //      "postgres://postgres:postgres@localhost:5432/postgres"
            //  );
        }
    }
    */