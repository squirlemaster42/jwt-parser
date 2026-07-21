use std::env;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::Sha256;
use hmac::{Hmac, KeyInit, Mac};

type HmacSha256 = Hmac<Sha256>;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO parse args better
    let token = &args[1];
    let secret = &args[2];

    let split_token: Vec<&str> = token.split('.').collect();
    let header = split_token[0];
    let payload = split_token[1];
    let signature = split_token[2];
    let json_str = decode_base64_json(header);
    println!("======== Header ========\n{}", json_str);

    let json_str = decode_base64_json(payload);
    println!("======== Payload ========\n{}", json_str);

    println!("======== Valid ========\n{}", validate_jwt(header, payload, signature, secret));
}

fn decode_base64_json(input: &str) -> std::string::String {
    let decoded_vec = match URL_SAFE_NO_PAD.decode(input) {
        Ok(v) => v,
        Err(e) => panic!("Base64 decode failed: {}", e),
    };

    let decoded_string = match str::from_utf8(&decoded_vec) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let json_obj: serde_json::Value = match serde_json::from_str(decoded_string) {
        Ok(v) => v,
        Err(e) => panic!("Could not convert string to json: {}", e),
    };

    let json_str: String = match serde_json::to_string_pretty(&json_obj) {
        Ok(v) => v,
        Err(e) => panic!("Failed to format json: {}", e),
    };

    // The return here not being needed is stupid...
    json_str
}

fn decode_jwt_signature(signature: &str) -> Vec<u8> {
    URL_SAFE_NO_PAD.decode(signature)
        .expect("Failed to decode signature")
}

fn validate_jwt(header: &str, payload: &str, signature: &str, secret: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");

    mac.update(format!("{}.{}", header, payload).as_bytes());

    match mac.verify_slice(&decode_jwt_signature(signature)) {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::validate_jwt;


    #[test]
    fn test_validate_jwt() {
        let header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let payload = "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0";
        let signature = "KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30";
        let secret = "a-string-secret-at-least-256-bits-long";

        assert!(validate_jwt(header, payload, signature, secret))
    }
}
