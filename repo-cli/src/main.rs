use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf;
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
   
   
}
