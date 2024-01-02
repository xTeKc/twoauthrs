/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

mod xcli;
use xcli::cli;

#[tokio::main]
async fn main() {
    cli::cli_main().await;
}
