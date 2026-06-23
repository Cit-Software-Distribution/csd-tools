use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Install,
}

#[derive(Parser)]
#[command(name = "csd", author, version, about = "CSD Tooling Ecosystem")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Install => println!("Buscar pelo pacote no manifesto"),
    }
}
