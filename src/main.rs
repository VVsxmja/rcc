mod cli;
mod lexical_analysis;
mod preprocessing;
mod syntax_analysis;

async fn rcc_main() {
    use clap::Parser;
    let cli = cli::Cli::parse();
    if let Err(e) = cli.execute().await {
        tracing::error!("Compiler error: {e}");
    }
}

fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_ansi(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    Ok(runtime.block_on(rcc_main()))
}
