use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{lexical_analysis, preprocessing, syntax_analysis};

#[derive(Parser)]
#[command(version)]
pub(crate) struct Cli {
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
}

impl Cli {
    pub(crate) async fn execute(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Preprocess { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                println!("{code}");
            }
            Commands::Lex { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                println!("{tokens:?}");
            }
            Commands::Syntax { file } => {
                let code = tokio::fs::read_to_string(file).await?;
                let code = preprocessing::remove_comment(&code)?;
                let tokens = lexical_analysis::extract_tokens(&code)?;
                let unit = syntax_analysis::parse(&tokens)?;
                println!("Parse result {unit:?}");
            }
        }
        Ok(())
    }
}
