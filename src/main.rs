mod protos;
use protobuf::Message;
use protos::otpm::{MigrationPayload, migration_payload};
use data_encoding::{BASE64, BASE32};
use urlencoding::{encode, decode};
use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "otpmdec")]
#[command(author = "crane@cranejin.com")]
#[command(version = "1.0")]
#[command(about = "Decode Google Authenticator migration data as \"issuer/name: secret\" lines to stdout.\n  Read input from stdin - you can either manually type/paste or pipe to it.", long_about=None)]
struct Cli {
    /// Print in Key Uri Format(https://github.com/google/google-authenticator/wiki/Key-Uri-Format) instead
    #[arg(long,short)]
    uri: bool,
}

fn main() {
    let cli = Cli::parse();
    do_the_work(cli.uri);
}

fn do_the_work(uri: bool) {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Error read input");

    let (_, data) = data.split_once("data=").expect("In valid input, should be something like: QR-Code:otpauth-migration://offline?data=CiYKFIAZV...");
    let data = decode(data).expect("Failed to perform url decoding");
    let data = data.trim_end();

    let payload = BASE64.decode(data.as_bytes()).expect("Failed to decode as base64");
    let payload = MigrationPayload::parse_from_bytes(&payload).expect("Unable to decode Protobuf message");

    let decode_func = if uri {
        to_key_uri
    } else {
        to_simple_foramt
    };

    payload.otp_parameters.iter().for_each(|otp| {
        let decoded = decode_func(otp);
        println!("{decoded}");
    });
}

fn to_simple_foramt(otp: &migration_payload::OtpParameters) -> String {
    let issuer = if otp.issuer.is_empty() { String::new() } else { format!("{}/", &otp.issuer) };
    let secret = BASE32.encode(&otp.secret);
    format!("{}{}: {}", issuer, otp.name, secret)
 }

const SHA1: &str = "SHA1";
const SHA256: &str = "SHA256";
const SHA512: &str = "SHA512";
const DIG_SIX: u8 = 6;
const DIG_EIGHT: u8 = 8;
const TOTP_TYPE: &str = "totp";
const HOTP_TYPE: &str = "hotp";
fn to_key_uri(otp: &migration_payload::OtpParameters) -> String {
    let mut params: Vec<String> = vec![];

    let issuer = if otp.issuer.is_empty() {
        String::new()
    } else {
        encode(&otp.issuer).to_string()
    };
    params.push(format!("issuer={}", &issuer));
    params.push(format!("secret={}", BASE32.encode(&otp.secret)));

    let algorithm = match otp.algorithm.enum_value_or_default() {
        migration_payload::Algorithm::ALGORITHM_SHA1 => Some(SHA1),
        migration_payload::Algorithm::ALGORITHM_SHA256 => Some(SHA256),
        migration_payload::Algorithm::ALGORITHM_SHA512 => Some(SHA512),
        _ => None,
    };
    if let Some(algo)= algorithm {
        params.push(format!("algorithm={}", algo));
    }

    let digits = match otp.digits.enum_value_or_default() {
        migration_payload::DigitCount::DIGIT_COUNT_SIX => Some(DIG_SIX),
        migration_payload::DigitCount::DIGIT_COUNT_EIGHT => Some(DIG_EIGHT),
        _ => None,
    };
    if let Some(d) = digits {
        params.push(format!("digits={}", d));
    }

    let otp_type = match otp.type_.enum_value_or_default() {
        migration_payload::OtpType::OTP_TYPE_HOTP => HOTP_TYPE,
        migration_payload::OtpType::OTP_TYPE_TOTP => TOTP_TYPE,
        _ => TOTP_TYPE,
    };
    if otp_type == "hotp" {
        params.push(format!("counter={}", otp.counter));
    }

    let name = encode(&otp.name).to_string();
    let label = if issuer.is_empty() {
        name
    } else {
        format!("{}:{}", &issuer, &name)
    };

    let params = params.join("&");

    format!("otpauth://{}/{}?{}", otp_type, &label, &params)
}
