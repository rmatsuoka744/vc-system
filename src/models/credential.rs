use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CredentialRequest {
    pub context: Vec<String>,
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub credential_subject: serde_json::Value,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CredentialResponse {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub credential_subject: serde_json::Value,
    pub proof: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct IssuerMetadata {
    pub id: String,
    pub name: String,
    pub public_key: PublicKeyInfo,
}

#[derive(Deserialize, Serialize)]
pub struct PublicKeyInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub public_key_multibase: String,
}

#[derive(Deserialize, Serialize)]
pub struct PresentationRequest {
    pub credential_ids: Vec<String>,
    pub challenge: String,
    pub domain: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifiablePresentation {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(rename = "type")]
    pub presentation_type: Vec<String>,
    pub verifiable_credential: Vec<CredentialResponse>,
    pub proof: Option<serde_json::Value>,
}