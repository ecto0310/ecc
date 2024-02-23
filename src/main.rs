mod analyze;
mod file;
mod generate;
mod parse;
mod tokenize;

use std::{fs::File, io::BufWriter, rc::Rc};

use crate::{
    analyze::analyzer::Analyzer,
    file::{file_info::FileInfo, file_stream::FileStream},
    generate::generator::Generator,
    parse::parser::Parser,
    tokenize::{token_stream::TokenStream, tokenizer::Tokenizer},
};

fn main() {
    let source_paths: Vec<String> = std::env::args().collect();
    let source_paths = &source_paths[1..];
    if source_paths.is_empty() {
        eprintln!("Not select source file");
    }
    for source_path in source_paths {
        let (file_info, output_buf) = match new_compile_info(source_path.to_string()) {
            Ok(compile_info) => compile_info,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };
        match compile(file_info, output_buf) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };
    }
}

fn compile(file_info: Rc<FileInfo>, mut output_buf: BufWriter<File>) -> anyhow::Result<()> {
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

fn new_compile_info(source_path: String) -> anyhow::Result<(Rc<FileInfo>, BufWriter<File>)> {
    let file_info = Rc::new(FileInfo::new(source_path.to_string())?);
    let output_file_name = source_path + ".s";
    let output_file = File::create(output_file_name)?;
    let output_buf = BufWriter::new(output_file);
    Ok((file_info, output_buf))
}
