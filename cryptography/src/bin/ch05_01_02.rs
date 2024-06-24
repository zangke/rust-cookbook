//! Filename: ch05_01_02.rs
//! Description: 使用 HMAC 摘要对消息进行签名和验证
//! Date: 2024/06/12 21:49:23
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{hmac, rand};

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
