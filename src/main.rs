use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self};
use std::io::{self, Read, Seek, Write};
use std::process;
use tar::Archive;
use tempfile::NamedTempFile;

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

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    version: String,
    url: String,
    checksum: String,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    programs: HashMap<String, Package>,
}

async fn download_package(package: &Package) -> Result<NamedTempFile, Box<dyn Error>> {
    let cache_dir = dirs::cache_dir().unwrap_or_else(|| {
        eprintln!("Couldn't find CACHE directory");
        process::exit(1);
    });
    let mut tempfile = tempfile::NamedTempFile::new_in(cache_dir)?;

    let response = reqwest::get(&package.url).await?;

    if !response.status().is_success() {
        eprintln!(
            "failed to download: server response status {}",
            response.status()
        );
        process::exit(1);
    }

    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        tempfile.write_all(&chunk)?;
    }

    tempfile.flush()?;

    println!("Download successful!");

    Ok(tempfile)
}

fn check_sum(temp_file: &mut NamedTempFile) -> io::Result<String> {
    let mut buffer = [0u8; 4096];
    let mut hasher = Sha256::new();
    temp_file.rewind()?;

    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

fn write_bin_file(temp_file: &mut NamedTempFile, bin_name: &str) -> Result<(), std::io::Error> {
    println!("Writing program to BIN path");
    let dir = dirs::executable_dir().unwrap_or_else(|| {
        eprintln!("Couldn't find BIN directory");
        process::exit(1);
    });

    temp_file.rewind()?;

    let tar_gz = temp_file.as_file();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    for entry_result in archive.entries()? {
        let mut entry = entry_result?;

        let mut final_path = dir.clone();
        final_path.push(bin_name);

        entry.unpack(final_path)?;
    }

    println!("Program was successfully written to BIN path. Make sure that BIN is in your PATH");

    Ok(())
}

async fn execute_routine(package: &Package, name: &str) -> Result<(), Box<dyn Error>> {
    println!("Starting download of {name:?}");
    let mut temp_file = download_package(package).await?;
    println!("Hashing donwloaded file");
    let checksum = check_sum(&mut temp_file)?;

    if checksum != package.checksum {
        eprintln!("Checksum didn't correspond, package changed");
        process::exit(1);
    } else {
        println!("Checksum ");
        write_bin_file(&mut temp_file, name)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| {
        eprintln!("Couldn't find local DATA directory.");
        process::exit(1);
    });

    dir.push("csd/manifest.json");

    let content = fs::read_to_string(dir)?;
    let manifest: Manifest = serde_json::from_str(&content)?;

    match &args.command {
        Commands::Install { package } => {
            if let Some(p) = manifest.programs.get(package) {
                println!("Package {package:?} was found");
                execute_routine(p, package).await?;
            } else {
                println!("Package {package:?} wasn't found");
            }
        }
    }
    Ok(())
}
