/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

use qr2term::print_qr;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha512};

pub fn core_main() -> String {
    let password: &[u8] = b"ThisIsTheSecretPassword";

    let seconds: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let result: String = totp::<Sha512>(password, seconds);
    assert_eq!(8, result.len());

    println!("result var: {}", result);
    print_qr(&result).unwrap();

    result
}
