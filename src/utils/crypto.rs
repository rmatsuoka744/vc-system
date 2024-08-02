use crate::models::credential::PublicKeyInfo;
use crate::utils::key_manager::{FileKeyManager, KeyManager};
use base58::{FromBase58, ToBase58};
use chrono::Utc;
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use log::{debug, error};
use rand::rngs::OsRng;
use serde_json::Value;

pub fn generate_new_keypair() -> (String, String) {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public.to_bytes().to_base58();
    let private_key = keypair.secret.to_bytes().to_base58();
    (public_key, private_key)
}

fn get_key_manager() -> impl KeyManager {
    FileKeyManager::new("keys/keys.json".to_string())
}

pub fn get_public_key_info() -> Result<PublicKeyInfo, String> {
    let key_manager = get_key_manager();
    let public_key = key_manager.get_public_key()?;
    Ok(PublicKeyInfo {
        id: "did:example:123#key-1".to_string(),
        key_type: "Ed25519VerificationKey2020".to_string(),
        public_key_multibase: format!("z{}", public_key.to_bytes().to_base58()),
    })
}

fn sort_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted_map.insert(key.clone(), sort_json(&map[key]));
            }
            Value::Object(sorted_map)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_json).collect()),
        _ => value.clone(),
    }
}


pub fn sign_json(json: &Value) -> Result<Value, String> {
    let key_manager = get_key_manager();
    let keypair = key_manager.get_keypair()?;
    let sorted_json = sort_json(json);
    let message = serde_json::to_string(&sorted_json).map_err(|e| e.to_string())?;
    debug!("Signing message: {}", message);
    let signature = keypair.sign(message.as_bytes());

    debug!("Signature created: {}", signature.to_bytes().to_base58());

    Ok(serde_json::json!({
        "type": "Ed25519Signature2020",
        "created": Utc::now().to_rfc3339(),
        "verificationMethod": "did:example:123#key-1",
        "proofPurpose": "assertionMethod",
        "proofValue": signature.to_bytes().to_base58()
    }))
}

pub fn verify_signature<T: serde::Serialize>(data: &T, proof: &Value) -> Result<bool, String> {
    debug!("Verifying signature");
    debug!("Proof: {:?}", proof);

    let sorted_data = sort_json(&serde_json::to_value(data).map_err(|e| e.to_string())?);
    let message = serde_json::to_string(&sorted_data).map_err(|e| e.to_string())?;
    debug!("Serialized data to verify: {}", message);

    let signature_base58 = proof["proofValue"].as_str().ok_or("Invalid proof value")?;
    let signature_bytes = signature_base58
        .from_base58()
        .map_err(|_| "Invalid base58 encoding".to_string())?;
    let signature = Signature::from_bytes(&signature_bytes).map_err(|e| e.to_string())?;

    let key_manager = get_key_manager();
    let public_key = key_manager.get_public_key()?;
    debug!(
        "Using public key for verification: {:?}",
        public_key.to_bytes().to_base58()
    );

    match public_key.verify(message.as_bytes(), &signature) {
        Ok(_) => {
            debug!("Signature verification successful");
            Ok(true)
        }
        Err(e) => {
            error!("Signature verification failed: {}", e);
            Err(e.to_string())
        }
    }
}
