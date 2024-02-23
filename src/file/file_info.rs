use std::io::Read;

#[derive(Debug, Clone)]
pub struct FileInfo {
    name: String,
    code: String,
}

impl FileInfo {
    pub fn new(name: String) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(&name)?;

        let mut code = String::new();
        file.read_to_string(&mut code)?;
        code.push('\n');
        Ok(Self { name, code })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }
}
