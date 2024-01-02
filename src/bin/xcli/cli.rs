/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

use clap::Parser;
use tf::xcore::core::core_main;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'g', value_name = "GEN", short)]
    generate: String,
}

pub async fn cli_main() {
    let args = Args::parse();

    let result = core_main();
    let match_cli = compare(args.generate, result).await;
    match match_cli {
        true => println!("Matches!"),
        false => (), //println!("No Match"),
    }
}

async fn compare(x: String, y: String) -> bool {
    x == y
}
