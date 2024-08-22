use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::{Sha256, Digest};
use serde_json::Value;

pub fn create_salt(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&result[..16]) // 最初の16バイトを使用
}

pub fn create_disclosure(salt: &str, claim_name: &str, claim_value: &Value) -> String {
    let value_str = match claim_value {
        Value::String(s) => s.clone(),
        _ => claim_value.to_string().trim_matches('"').to_string(),
    };
    format!("{}.{}.{}", salt, claim_name, value_str)
}

pub fn hash_disclosure(disclosure: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(disclosure);
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(result)
}