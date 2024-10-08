use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialRequest {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(rename = "type")]
    pub types: Vec<String>,
    pub issuer: String,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialResponse {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub types: Vec<String>,
    pub issuer: String,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd_jwt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuerMetadata {
    pub id: String,
    pub name: String,
    #[serde(rename = "publicKey")]
    pub public_key: PublicKeyInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicKeyInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub key_type: String,
    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresentationRequest {
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<String>,
    pub domain: String,
    pub challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiablePresentation {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(rename = "type")]
    pub types: Vec<String>,
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<CredentialResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SDJWTCredentialResponse {
    pub verifiable_credential: CredentialResponse,
    pub sd_jwt: String,
    pub disclosures: Vec<String>,
}
