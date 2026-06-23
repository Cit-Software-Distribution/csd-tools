use clap::{Parser, Subcommand};
use dirs;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Subcommand)]
enum Commands {
    Install { package: String },
}

#[derive(Parser)]
#[command(name = "csd", author, version, about = "CSD Tooling Ecosystem")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize)]
struct Package {
    version: String,
    url: String,
    checksum: String,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    programs: HashMap<String, Package>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| {
        eprintln!("Couldn't find local DATA path.");
        process::exit(1);
    });

    dir.push("csd/manifest.json");

    let content = fs::read_to_string(dir)?;
    let manifest: Manifest = serde_json::from_str(&content)?;

    match &args.command {
        Commands::Install { package } => {
            if let Some(p) = manifest.programs.get(package) {
                println!("Package {package:?} was found");
            } else {
                println!("Package {package:?} wasn't found");
            }
        }
    }
    Ok(())
}
