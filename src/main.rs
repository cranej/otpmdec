mod protos;
use protobuf::Message;
use protos::otpm::MigrationPayload;
use data_encoding::{BASE64, BASE32};
use urlencoding::decode;
use std::{io,env};

fn main() {
    match env::args().nth(1) {
        Some(arg) if arg == "--help" || arg == "-h" => {
            println!("Decode Google Authenticator migration data as \"issuer/name: secret\" lines to stdout.");
            println!("  Read input from stdin - you can either manually type/paste or pipe to it.");
        },
        _ => {
            do_the_work()
        }
    }
}

fn do_the_work() {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Error read input");

    let (_, data) = data.split_once("data=").expect("In valid input, should be something like: QR-Code:otpauth-migration://offline?data=CiYKFIAZV...");
    let decoded_data = decode(data).expect("Failed to perform url decoding");
    let decoded_data = decoded_data.trim_end();

    let payload_bytes = BASE64.decode(decoded_data.as_bytes()).expect("Failed to decode as base64");
    let payload = MigrationPayload::parse_from_bytes(&payload_bytes).unwrap();
    for item in &payload.otp_parameters {
        let issuer = if item.issuer.is_empty() { String::new() } else { format!("{}/", &item.issuer) };
        let secret = BASE32.encode(&item.secret);
        println!("{}{}: {}", &issuer, &item.name, &secret);
    }
}
