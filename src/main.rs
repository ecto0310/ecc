mod error;
mod file;
mod tokenize;

use error::error::Error;
use file::file_info::FileInfo;
use file::file_stream::FileStream;
use std::rc::Rc;
use tokenize::tokenizer::Tokenizer;

fn main() -> Result<(), Error> {
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
                eprintln!("{:?}", err);
            }
        };
    }
    Ok(())
}

fn compile(file_info: Rc<FileInfo>) -> Result<(), Error> {
    let file_stream = FileStream::new(file_info);
    let mut tokenizer = Tokenizer::new(file_stream);
    let tokens = tokenizer.tokenize()?;
    print!("{tokens:?}");
    Ok(())
}
