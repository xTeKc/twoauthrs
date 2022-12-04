#![allow(unused)]
use clap::{Parser, Subcommand};
use qr2term::print_qr;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha512};

/// Manage inventory
#[derive(Parser)]
#[command(disable_help_subcommand(true), author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate new auth token
    G { input: String },
}

#[tokio::main]
async fn main() {
    // Negotiated between you and the authenticating service.
    let password: &[u8] = b"ThisIsTheSecretPassword";

    // The number of seconds since the Unix Epoch.
    let seconds: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Specify the desired Hash algorithm via a type parameter.
    let result: String = totp::<Sha512>(password, seconds);
    assert_eq!(8, result.len());

    println!("result var: {}", result);

    print_qr(&result);

    let cli = Cli::parse();

    let match_cli = match &cli.command {
        Commands::G { input } => compare(input.to_owned(), result),
    };

    //println!("match_cli var: {:#?}", match_cli.await);

    match match_cli.await {
        true => println!("Matches!"),
        false => (), //println!("No Match"),
    }
}

async fn compare(x: String, y: String) -> bool {
    x == y
}
