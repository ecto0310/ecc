use crate::error::Error;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct FileInfo {
    name: String,
    code: String,
}

impl FileInfo {
    pub fn new(name: String) -> Result<Self, Error> {
        let mut file = match std::fs::File::open(&name) {
            Ok(file) => file,
            Err(error) => return Err(Error::new_io(error)),
        };

        let mut code = String::new();
        match file.read_to_string(&mut code) {
            Ok(_) => {
                code.push('\n');
                Ok(Self { name, code })
            }
            Err(error) => Err(Error::new_io(error)),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }
}
