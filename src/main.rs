// #![allow(warnings)]

mod cli;
mod lexical_analysis;
mod preprocessing;
mod semantic_analysis;
mod syntax_analysis;
mod object_file_generator;

async fn rcc_main() -> anyhow::Result<()> {
    use clap::Parser;
    let cli = cli::Cli::parse();
    cli.execute().await
}

fn main() -> anyhow::Result<()> {
    use tracing_subscriber::{filter::LevelFilter, EnvFilter};
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_ansi(true)
        .with_target(false)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::OFF.into())
                .from_env_lossy(),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(rcc_main())
}
