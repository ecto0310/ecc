mod analyze;
mod error;
mod file;
mod generate;
mod parse;
mod tokenize;

use analyze::analyzer::Analyzer;
use error::error::Error;
use file::file_info::FileInfo;
use file::file_stream::FileStream;
use generate::generator::Generator;
use parse::parser::Parser;
use std::fs::File;
use std::io::BufWriter;
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
        let (file_info, output_buf) = new_compile_info(source_path.to_string())?;
        match compile(file_info, output_buf) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{:?}", err);
            }
        };
    }
    Ok(())
}

fn compile(file_info: Rc<FileInfo>, mut output_buf: BufWriter<File>) -> Result<(), Error> {
    let file_stream = FileStream::new(file_info);
    let mut tokenizer = Tokenizer::new(file_stream);
    let tokens = tokenizer.tokenize()?;

    let mut token_stream = TokenStream::new(tokens);
    let mut parser = Parser::new();
    let syntax_tree = parser.parse(&mut token_stream)?;

    let mut analyzer = Analyzer::new();
    let gen_tree = analyzer.analyze(syntax_tree)?;

    let mut generator = Generator::new();
    generator.generate(&mut output_buf, gen_tree)?;
    Ok(())
}

fn new_compile_info(source_path: String) -> Result<(Rc<FileInfo>, BufWriter<File>), Error> {
    let file_info = Rc::new(FileInfo::new(source_path.to_string())?);
    let output_file_name = String::from(source_path) + ".s";
    let output_file = File::create(output_file_name)?;
    let output_buf = BufWriter::new(output_file);
    Ok((file_info, output_buf))
}
