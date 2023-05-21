mod analyze;
mod error;
mod file;
mod parse;
mod tokenize;

use analyze::analyzer::Analyzer;
use error::error::Error;
use file::file_info::FileInfo;
use file::file_stream::FileStream;
use parse::parser::Parser;
use std::rc::Rc;
use tokenize::token_stream::TokenStream;
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

    let mut token_stream = TokenStream::new(tokens);
    let mut parser = Parser::new();
    let syntax_tree = parser.parse(&mut token_stream)?;

    let mut analyzer = Analyzer::new();
    let gen_tree = analyzer.analyze(syntax_tree)?;
    print!("{gen_tree:?}");
    Ok(())
}
