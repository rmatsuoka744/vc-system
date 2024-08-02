use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use base58::FromBase58;
use log::debug;

#[derive(Serialize, Deserialize)]
struct KeyPair {
    public_key: String,
    private_key: String,
}

pub trait KeyManager {
    fn get_keypair(&self) -> Result<Keypair, String>;
    fn get_public_key(&self) -> Result<PublicKey, String>;
}

pub struct FileKeyManager {
    file_path: String,
}

impl FileKeyManager {
    pub fn new(file_path: String) -> Self {
        FileKeyManager { file_path }
    }
}

impl KeyManager for FileKeyManager {
    fn get_keypair(&self) -> Result<Keypair, String> {
        let mut file = File::open(&self.file_path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        let key_pair: KeyPair = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

        debug!("Loaded public key: {}", key_pair.public_key);
        debug!("Loaded private key: {}", key_pair.private_key);

        let secret = SecretKey::from_bytes(&key_pair.private_key.from_base58().map_err(|e| format!("Invalid private key: {:?}", e))?)
            .map_err(|e| e.to_string())?;
        let public = PublicKey::from_bytes(&key_pair.public_key.from_base58().map_err(|e| format!("Invalid public key: {:?}", e))?)
            .map_err(|e| e.to_string())?;

        Ok(Keypair { secret, public })
    }

    fn get_public_key(&self) -> Result<PublicKey, String> {
        let keypair = self.get_keypair()?;
        Ok(keypair.public)
    }
}
