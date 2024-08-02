use base58::{FromBase58, ToBase58};
use chrono::Utc;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer, Verifier};
use serde_json::Value;

// 鍵管理のためのトレイト
pub trait KeyManager {
    fn get_keypair(&self) -> &Keypair;
    fn get_public_key(&self) -> &PublicKey;
}

// 固定鍵を使用する実装
pub struct FixedKeyManager {
    keypair: Keypair,
}

impl FixedKeyManager {
    pub fn new() -> Self {
        // 注意: 実際の運用では、この方法は使用しないでください
        let secret = SecretKey::from_bytes(&[
            // 32バイトの秘密鍵をここに記述
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ])
        .expect("Invalid secret key");
        let public = PublicKey::from(&secret);
        FixedKeyManager {
            keypair: Keypair { secret, public },
        }
    }
}

impl KeyManager for FixedKeyManager {
    fn get_keypair(&self) -> &Keypair {
        &self.keypair
    }

    fn get_public_key(&self) -> &PublicKey {
        &self.keypair.public
    }
}

// グローバルな KeyManager インスタンス
lazy_static::lazy_static! {
    static ref KEY_MANAGER: FixedKeyManager = FixedKeyManager::new();
}

pub fn get_public_key_info() -> crate::models::credential::PublicKeyInfo {
    let public_key = KEY_MANAGER.get_public_key();
    crate::models::credential::PublicKeyInfo {
        id: "did:example:123#key-1".to_string(),
        key_type: "Ed25519VerificationKey2020".to_string(),
        public_key_multibase: format!("z{}", public_key.as_bytes().to_base58()),
    }
}

pub fn sign_json(json: &Value) -> Result<Value, String> {
    let keypair = KEY_MANAGER.get_keypair();
    let message = serde_json::to_string(json).map_err(|e| e.to_string())?;
    let signature = keypair.sign(message.as_bytes());

    Ok(serde_json::json!({
        "type": "Ed25519Signature2020",
        "created": Utc::now().to_rfc3339(),
        "verificationMethod": "did:example:123#key-1",
        "proofPurpose": "assertionMethod",
        "proofValue": signature.to_bytes().to_base58()
    }))
}

pub fn verify_signature<T: serde::Serialize>(data: &T, proof: &Value) -> Result<bool, String> {
    let message = serde_json::to_string(data).map_err(|e| e.to_string())?;
    let signature_base58 = proof["proofValue"].as_str().ok_or("Invalid proof value")?;
    let signature_bytes = signature_base58.from_base58().map_err(|_| "Invalid base58 encoding".to_string())?;
    let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes).map_err(|e| e.to_string())?;

    let public_key = KEY_MANAGER.get_public_key();

    public_key.verify(message.as_bytes(), &signature).map_err(|e| e.to_string())?;
    Ok(true)
}