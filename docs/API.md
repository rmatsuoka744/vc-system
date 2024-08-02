# 検証可能な資格証明（Verifiable Credentials）API ドキュメント

このドキュメントは、W3C 検証可能な資格証明データモデル 1.1 に準拠した Verifiable Credentials API の使用方法を説明します。

## ベース URL

すべての API リクエストは以下の URL に対して行われます：`http://localhost:8080`

## Issuer API

### 1. VCの発行

**エンドポイント:** `POST /issuer/credentials`

**リクエスト本文:**
```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "https://example.edu/issuers/14",
  "issuanceDate": "2023-06-01T19:23:24Z",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  }
}
```

**期待される応答:**
```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.edu/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "https://example.edu/issuers/14",
  "issuanceDate": "2023-06-01T19:23:24Z",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-06-01T19:23:24Z",
    "verificationMethod": "https://example.edu/issuers/14#key-1",
    "proofPurpose": "assertionMethod",
    "proofValue": "z58DAdkxz7A..."
  }
}
```

### 2. Issuerのメタデータ取得

**エンドポイント:** `GET /issuer/metadata`

**期待される応答:**
```json
{
  "id": "did:example:123",
  "name": "Example University",
  "publicKey": {
    "id": "did:example:123#key-1",
    "type": "Ed25519VerificationKey2020",
    "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  }
}
```

## Holder API

### 1. VCの保存

**エンドポイント:** `POST /holder/credentials`

**リクエスト本文:** 
```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.edu/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "https://example.edu/issuers/14",
  "issuanceDate": "2023-06-01T19:23:24Z",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-06-01T19:23:24Z",
    "verificationMethod": "https://example.edu/issuers/14#key-1",
    "proofPurpose": "assertionMethod",
    "proofValue": "z58DAdkxz7A..."
  }
}
```

**期待される応答:**
```json
{
  "id": "91cf9bc2-1901-4948-9889-4f8819e379e2",
  "status": "stored"
}
```

### 2. 保存されたVCの取得

**エンドポイント:** `GET /holder/credentials`

**期待される応答:**
```json
[
  {
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "https://example.edu/issuers/14",
    "issuanceDate": "2023-06-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science in Mechanical Engineering"
      }
    },
    "proof": {
      "type": "Ed25519Signature2020",
      "created": "2023-06-01T19:23:24Z",
      "verificationMethod": "https://example.edu/issuers/14#key-1",
      "proofPurpose": "assertionMethod",
      "proofValue": "z58DAdkxz7A..."
    }
  }
]
```

### 3. プレゼンテーションの作成

**エンドポイント:** `POST /holder/presentations`

**リクエスト本文:**
```json
{
  "verifiableCredential": ["http://example.edu/credentials/3732"],
  "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
  "domain": "example.com"
}
```

**期待される応答:**
```json
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],
  "type": ["VerifiablePresentation"],
  "verifiableCredential": [
    {
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "http://example.edu/credentials/3732",
      "type": ["VerifiableCredential", "UniversityDegreeCredential"],
      "issuer": "https://example.edu/issuers/14",
      "issuanceDate": "2023-06-01T19:23:24Z",
      "credentialSubject": {
        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
        "degree": {
          "type": "BachelorDegree",
          "name": "Bachelor of Science in Mechanical Engineering"
        }
      },
      "proof": {
        "type": "Ed25519Signature2020",
        "created": "2023-06-01T19:23:24Z",
        "verificationMethod": "https://example.edu/issuers/14#key-1",
        "proofPurpose": "assertionMethod",
        "proofValue": "z58DAdkxz7A..."
      }
    }
  ]
}
```

## Verifier API

### 1. VCの検証

**エンドポイント:** `POST /verifier/verify/credential`

**リクエスト本文:**
```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.edu/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "https://example.edu/issuers/14",
  "issuanceDate": "2023-06-01T19:23:24Z",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-06-01T19:23:24Z",
    "verificationMethod": "https://example.edu/issuers/14#key-1",
    "proofPurpose": "assertionMethod",
    "proofValue": "z58DAdkxz7A..."
  }
}
```

**期待される応答:**
```json
{
  "errors": [],
  "verified": true
}
```

### 2. プレゼンテーションの検証

**エンドポイント:** `POST /verifier/verify/presentation`

**リクエスト本文:**
```json
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],
  "type": ["VerifiablePresentation"],
  "verifiableCredential": [
    {
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "http://example.edu/credentials/3732",
      "type": ["VerifiableCredential", "UniversityDegreeCredential"],
      "issuer": "https://example.edu/issuers/14",
      "issuanceDate": "2023-06-01T19:23:24Z",
      "credentialSubject": {
        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
        "degree": {
          "type": "BachelorDegree",
          "name": "Bachelor of Science in Mechanical Engineering"
        }
      },
      "proof": {
        "type": "Ed25519Signature2020",
        "created": "2023-06-01T19:23:24Z",
        "verificationMethod": "https://example.edu/issuers/14#key-1",
        "proofPurpose": "assertionMethod",
        "proofValue": "z58DAdkxz7A..."
      }
    }
  ]
}
```

**期待される応答:**
```json
{
  "errors": [],
  "verified": true
}
```