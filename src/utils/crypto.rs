// src/utils/crypto.rs
use crate::models::credential::PublicKeyInfo;
use crate::utils::error::UtilsError;
use crate::utils::key_manager::{FileKeyManager, KeyManager};
use base58::{FromBase58, ToBase58};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::Utc;
use ed25519_dalek::{PublicKey, Signature, Signer, Verifier};
use log::debug;
use serde_json::Value;

fn get_key_manager() -> impl KeyManager {
    FileKeyManager::new("keys/keys.json".to_string())
}

pub fn get_public_key() -> Result<PublicKey, UtilsError> {
    get_key_manager()
        .get_public_key()
        .map_err(|_| UtilsError::SignatureError("Failed to get public key".to_string()))
}

pub fn get_public_key_info() -> Result<PublicKeyInfo, UtilsError> {
    let key_manager = get_key_manager();
    let public_key = key_manager
        .get_public_key()
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?; // ここでエラーを `UtilsError` に変換
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

pub fn sign_vc(json: &Value) -> Result<Value, UtilsError> {
    let key_manager = get_key_manager();
    let keypair = key_manager
        .get_keypair()
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?;
    let sorted_json = sort_json(json);
    let message = serde_json::to_string(&sorted_json)
        .map_err(|e| UtilsError::JsonSerializationError(e.to_string()))?;
    debug!("Signing VC message: {}", message);

    let signature = keypair.sign(message.as_bytes());
    debug!("VC Signature created: {}", signature.to_bytes().to_base58());

    Ok(serde_json::json!({
        "type": "Ed25519Signature2020",
        "created": Utc::now().to_rfc3339(),
        "verificationMethod": "did:example:123#key-1",
        "proofPurpose": "assertionMethod",
        "proofValue": signature.to_bytes().to_base58(),
    }))
}

pub fn sign_sd_jwt(json: &Value) -> Result<String, UtilsError> {
    let key_manager = get_key_manager();
    let keypair = key_manager
        .get_keypair()
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?;
    let sorted_json = sort_json(json);
    let message = serde_json::to_string(&sorted_json)
        .map_err(|e| UtilsError::JsonSerializationError(e.to_string()))?;

    let header = serde_json::json!({
        "alg": "EdDSA",
        "typ": "JWT"
    });

    let header_encoded = URL_SAFE_NO_PAD.encode(
        serde_json::to_string(&header)
            .map_err(|e| UtilsError::JsonSerializationError(e.to_string()))?,
    );
    let payload_encoded = URL_SAFE_NO_PAD.encode(&message);

    let signature_input = format!("{}.{}", header_encoded, payload_encoded);
    let signature = keypair.sign(signature_input.as_bytes());
    let signature_encoded = URL_SAFE_NO_PAD.encode(signature.to_bytes());

    debug!("SD-JWT Signature created: {}", signature_encoded);

    Ok(format!(
        "{}.{}.{}",
        header_encoded, payload_encoded, signature_encoded
    ))
}

pub fn sign_json(json: &Value) -> Result<Value, UtilsError> {
    if json.get("_sd_alg").is_some() {
        sign_sd_jwt(json).map(Value::String)
    } else {
        sign_vc(json)
    }
}

pub fn verify_vc<T: serde::Serialize>(data: &T, proof: &Value) -> Result<bool, UtilsError> {
    let key_manager = get_key_manager();
    let public_key = key_manager
        .get_public_key()
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?; // ここでも同様にエラーを変換

    let message = serde_json::to_string(&sort_json(
        &serde_json::to_value(data)
            .map_err(|e| UtilsError::JsonSerializationError(e.to_string()))?,
    ))
    .map_err(|e| UtilsError::JsonSerializationError(e.to_string()))?;
    let signature_base58 = proof["proofValue"]
        .as_str()
        .ok_or(UtilsError::SignatureError(
            "Invalid proof value".to_string(),
        ))?;
    let signature_bytes = signature_base58
        .from_base58()
        .map_err(|_| UtilsError::SignatureError("Invalid base58 encoding".to_string()))?;
    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?;

    public_key
        .verify(message.as_bytes(), &signature)
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?;
    Ok(true)
}

pub fn verify_sd_jwt(jwt: &str) -> Result<bool, UtilsError> {
    let public_key = get_public_key()?;
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return Err(UtilsError::SignatureError("Invalid JWT format".to_string()));
    }

    let signature_input = format!("{}.{}", parts[0], parts[1]);
    let signature_bytes = URL_SAFE_NO_PAD
        .decode(parts[2])
        .map_err(|_| UtilsError::SignatureError("Invalid base64 encoding".to_string()))?;
    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?;

    public_key
        .verify(signature_input.as_bytes(), &signature)
        .map_err(|e| UtilsError::SignatureError(e.to_string()))?;
    Ok(true)
}