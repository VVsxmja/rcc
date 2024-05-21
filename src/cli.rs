use std::path::PathBuf;

use clap::{Parser, Subcommand};
use inkwell::targets::FileType;

use crate::{
    lexical_analysis,
    object_file_generator::generate_object_file,
    preprocessing,
    semantic_analysis::{self, bitcode_to_string},
    syntax_analysis,
};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View the preprocessing result of a source file
    Preprocess { file: PathBuf },
    /// View the lexical analysis result of a source file
    Lex { file: PathBuf },
    /// View the syntax analysis result of a source file
    Syntax { file: PathBuf },
    /// View the semantic analysis result of a source file
    Semantic { file: PathBuf },
    /// Generate binary from a source file
    CompileBinary { file: PathBuf, output: PathBuf },
    /// Generate assembly from a source file
    CompileAssembly { file: PathBuf, output: PathBuf },
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Preprocess { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                println!("Preprocess result: {code}");
            }
            Commands::Lex { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                println!("Lexical analysis result: {tokens:?}");
            }
            Commands::Syntax { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                let unit = syntax_analysis::parse(&tokens)?;
                println!("Syntax analysis result: {unit:#?}");
            }
            Commands::Semantic { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                let unit = syntax_analysis::parse(&tokens)?;
                let bitcode = semantic_analysis::analysis(unit)?;
                let ir = bitcode_to_string(bitcode)?;
                println!("Semantic analysis result: {ir}");
            }
            Commands::CompileBinary { file, output } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                let unit = syntax_analysis::parse(&tokens)?;
                let bitcode = semantic_analysis::analysis(unit)?;
                generate_object_file(bitcode, &output, FileType::Object)?;
            }
            Commands::CompileAssembly { file, output } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                let unit = syntax_analysis::parse(&tokens)?;
                let bitcode = semantic_analysis::analysis(unit)?;
                generate_object_file(bitcode, &output, FileType::Assembly)?;
            }
        }
        Ok(())
    }
}
