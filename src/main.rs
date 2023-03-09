mod error;
mod file;

use std::rc::Rc;

use crate::file::file_info::FileInfo;
use error::CompileError;
use file::file_stream::FileStream;

fn main() -> Result<(), CompileError> {
    let source_paths: Vec<String> = std::env::args().collect();
    let source_paths = &source_paths[1..];
    if source_paths.is_empty() {
        eprintln!("Not select source file");
    }
    for source_path in source_paths {
        let file_info = Rc::new(FileInfo::new(source_path.to_string())?);
        match compile(file_info) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }
    Ok(())
}

fn compile(file_info: Rc<FileInfo>) -> Result<(), CompileError> {
    let mut file_stream = FileStream::new(file_info);
    Ok(())
}
