use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error!!!!!!!!!!!: {0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

impl std::conver::From<uuid::Error> Error {
    fn from(err: uuid::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Internal(err.to_string())
    }
}


pub struct SlyCooper {
    pub leo: String,
    pub location: DateTime
}

impl SlyCooper {
    pub fn set(&mut self, new_data: &str) {
        self.leo = new_data.to_string();
    }

    pub fn rev() {
        let s = String::new();
        let rev = s.chars().rev().collect::<String>();
        
    }
}