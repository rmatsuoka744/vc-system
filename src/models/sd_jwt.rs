use crate::models::credential::CredentialResponse;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SDJWTCredentialRequest {
    #[serde(rename = "credentialSubject")]
    pub credential_subject: Value,
    //pub selective_disclosure: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SDJWTCredentialResponse {
    pub verifiable_credential: CredentialResponse,
    pub sd_jwt: String,
    pub disclosures: Vec<String>,
}