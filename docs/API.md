# Verifiable Credentials API ドキュメンテーション

## 1. Issuer API

### 1.1.1 クレデンシャル発行

新しいVerifiable Credentialを発行します。

**エンドポイント:** `POST /issuer/credentials`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/issuer/credentials \
     -H "Content-Type: application/json" \
     -d '{
       "@context": [
         "https://www.w3.org/2018/credentials/v1",
         "https://www.w3.org/2018/credentials/examples/v1"
       ],
       "type": ["VerifiableCredential", "UniversityDegreeCredential"],
       "issuer": "did:example:123",
       "issuanceDate": "2023-06-01T19:23:24Z",
       "credentialSubject": {
         "id": "did:example:456",
         "name": "Alice Johnson",
         "degree": {
           "type": "BachelorDegree",
           "name": "Bachelor of Science in Mechanical Engineering"
         }
       }
     }'
```

**レスポンス例:**

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.edu/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "did:example:123",
  "issuanceDate": "2023-06-01T19:23:24Z",
  "credentialSubject": {
    "id": "did:example:456",
    "name": "Alice Johnson",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-06-01T19:23:24Z",
    "verificationMethod": "did:example:123#key-1",
    "proofPurpose": "assertionMethod",
    "proofValue": "z58DAdkxz7A..."
  }
}
```

### 1.1.2 SD-JWT-VCの発行

**エンドポイント:** `POST issuer/sd-jwt-credentials`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/issuer/sd-jwt-credentials \
-H "Content-Type: application/json" \
-d '{
    "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "type": [
        "VerifiableCredential",
        "UniversityDegreeCredential"
    ],
    "credentialSubject": {
        "id": "did:example:456",
        "given_name": "Alice",
        "family_name": "Johnson",
        "email": "em@il",
        "birthdate": "1/1",
        "degree": {
            "type": "BachelorDegree",
            "name": "Bachelor of Science in Mechanical Engineering"
        }
    }
}'
```

**レスポンス例:**

```bash
{
    "verifiable_credential": {
        "@context": [
            "https://www.w3.org/2018/credentials/v1"
        ],
        "id": "http://example.edu/credentials/bf1fbd5e-b71e-44ef-abeb-d739889a0eee",
        "type": [
            "VerifiableCredential",
            "SDJWTCredential"
        ],
        "issuer": "did:example:123",
        "issuanceDate": "2024-08-22T06:26:51.151680390+00:00",
        "credentialSubject": {
            "birthdate": "1/1",
            "degree": {
                "name": "Bachelor of Science in Mechanical Engineering",
                "type": "BachelorDegree"
            },
            "email": "em@il",
            "family_name": "Johnson",
            "given_name": "Alice",
            "id": "did:example:456"
        },
        "proof": {
            "created": "2024-08-22T06:26:51.152480517+00:00",
            "proofPurpose": "assertionMethod",
            "proofValue": "2yGjUxB816C3EJ6ybcgbJCRL4vyt7JucBz5ahd83EjANnaWgwNhBtL6PMeBxxtArzskocFFKoqCWBBhvdVqys7J8",
            "type": "Ed25519Signature2020",
            "verificationMethod": "did:example:123#key-1"
        }
    },
    "sd_jwt": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJfc2QiOlsiaUVURTRiNlhadnZCOG9FVFE3Sk96eklGRTJUTlpyY2pEUU9GZjRhOVhFUSIsIk9xMGNZNmh3U2JWZ3FvM0RwQlduSkVEMDR4WWh5eEZwRU8wWEl4M3RqZmsiXSwiX3NkX2FsZyI6InNoYS0yNTYiLCJkZWdyZWUiOnsibmFtZSI6IkJhY2hlbG9yIG9mIFNjaWVuY2UgaW4gTWVjaGFuaWNhbCBFbmdpbmVlcmluZyIsInR5cGUiOiJCYWNoZWxvckRlZ3JlZSJ9LCJmYW1pbHlfbmFtZSI6IkpvaG5zb24iLCJnaXZlbl9uYW1lIjoiQWxpY2UiLCJpYXQiOjE3MjQzMDgwMTEsImlkIjoiZGlkOmV4YW1wbGU6NDU2IiwiaXNzIjoiZGlkOmV4YW1wbGU6MTIzIiwidmN0IjoiU0RKV1RDcmVkZW50aWFsIn0.VodBLUKXqZfqagRieIDDbeyXVSB82jQXl3_DC8IQBdazWCJbQ566zIB66kmWdxsmdyrHCJtaqkTxA09MH5NRDg",
    "disclosures": [
        "w70I5CoZtx_x4oOzTGXyzA.birthdate.1/1",
        "EHcjQ7P835_zJATp2M_ymQ.email.em@il"
    ]
}
```

### 1.2 Issuerメタデータ取得

Issuerのメタデータを取得します。

**エンドポイント:** `GET /issuer/metadata`

**リクエスト例:**

```bash
curl http://localhost:8080/issuer/metadata
```

**レスポンス例:**

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

## 2. Holder API

### 2.1 クレデンシャル保存

受け取ったVerifiable Credentialを保存します。

**エンドポイント:** `POST /holder/credentials`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/holder/credentials \
     -H "Content-Type: application/json" \
     -d '{
       "@context": [
         "https://www.w3.org/2018/credentials/v1",
         "https://www.w3.org/2018/credentials/examples/v1"
       ],
       "id": "http://example.edu/credentials/3732",
       "type": ["VerifiableCredential", "UniversityDegreeCredential"],
       "issuer": "did:example:123",
       "issuanceDate": "2023-06-01T19:23:24Z",
       "credentialSubject": {
         "id": "did:example:456",
         "name": "Alice Johnson",
         "degree": {
           "type": "BachelorDegree",
           "name": "Bachelor of Science in Mechanical Engineering"
         }
       },
       "proof": {
         "type": "Ed25519Signature2020",
         "created": "2023-06-01T19:23:24Z",
         "verificationMethod": "did:example:123#key-1",
         "proofPurpose": "assertionMethod",
         "proofValue": "z58DAdkxz7A..."
       }
     }'
```

**レスポンス例:**

```json
{
  "status": "success",
  "message": "Credential stored successfully"
}
```

### 2.2 保存されたクレデンシャル一覧取得

Holderが保存しているクレデンシャルの一覧を取得します。

**エンドポイント:** `GET /holder/credentials`

**リクエスト例:**

```bash
curl http://localhost:8080/holder/credentials
```

**レスポンス例:**

```json
{
  "credentials": [
    {
      "id": "http://example.edu/credentials/3732",
      "type": ["VerifiableCredential", "UniversityDegreeCredential"],
      "issuer": "did:example:123",
      "issuanceDate": "2023-06-01T19:23:24Z"
    },
    {
      "id": "http://example.com/credentials/1234",
      "type": ["VerifiableCredential", "EmploymentCredential"],
      "issuer": "did:example:789",
      "issuanceDate": "2023-05-15T10:30:00Z"
    }
  ]
}
```

### 2.3 プレゼンテーション作成

保存されているクレデンシャルを使用してVerifiable Presentationを作成します。

**エンドポイント:** `POST /holder/presentations`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/holder/presentations \
     -H "Content-Type: application/json" \
     -d '{
       "verifiableCredential": ["http://example.edu/credentials/3732"],
       "domain": "example.com",
       "challenge": "1f44d55f-f161-4938-a659-f8026467f126"
     }'
```

**レスポンス例:**

```json
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],
  "type": ["VerifiablePresentation"],
  "verifiableCredential": [{
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "did:example:123",
    "issuanceDate": "2023-06-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:456",
      "name": "Alice Johnson",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science in Mechanical Engineering"
      }
    },
    "proof": {
      "type": "Ed25519Signature2020",
      "created": "2023-06-01T19:23:24Z",
      "verificationMethod": "did:example:123#key-1",
      "proofPurpose": "assertionMethod",
      "proofValue": "z58DAdkxz7A..."
    }
  }],
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-06-02T12:00:00Z",
    "verificationMethod": "did:example:456#key-1",
    "proofPurpose": "authentication",
    "proofValue": "z6MkhaXgBZD...",
    "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
    "domain": "example.com"
  }
}
```

## 3. Verifier API

### 3.1 クレデンシャル検証

Verifiable Credentialの検証を行います。

**エンドポイント:** `POST /verifier/credentials`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/verifier/credentials \
     -H "Content-Type: application/json" \
     -d '{
       "@context": [
         "https://www.w3.org/2018/credentials/v1",
         "https://www.w3.org/2018/credentials/examples/v1"
       ],
       "id": "http://example.edu/credentials/3732",
       "type": ["VerifiableCredential", "UniversityDegreeCredential"],
       "issuer": "did:example:123",
       "issuanceDate": "2023-06-01T19:23:24Z",
       "credentialSubject": {
         "id": "did:example:456",
         "name": "Alice Johnson",
         "degree": {
           "type": "BachelorDegree",
           "name": "Bachelor of Science in Mechanical Engineering"
         }
       },
       "proof": {
         "type": "Ed25519Signature2020",
         "created": "2023-06-01T19:23:24Z",
         "verificationMethod": "did:example:123#key-1",
         "proofPurpose": "assertionMethod",
         "proofValue": "z58DAdkxz7A..."
       }
     }'
```

**レスポンス例:**

```json
{
  "verified": true
}
```

### 3.2 プレゼンテーション検証

Verifiable Presentationの検証を行います。

**エンドポイント:** `POST /verifier/presentations`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/verifier/presentations \
     -H "Content-Type: application/json" \
     -d '{
       "@context": ["https://www.w3.org/2018/credentials/v1"],
       "type": ["VerifiablePresentation"],
       "verifiableCredential": [{
         "@context": [
           "https://www.w3.org/2018/credentials/v1",
           "https://www.w3.org/2018/credentials/examples/v1"
         ],
         "id": "http://example.edu/credentials/3732",
         "type": ["VerifiableCredential", "UniversityDegreeCredential"],
         "issuer": "did:example:123",
         "issuanceDate": "2023-06-01T19:23:24Z",
         "credentialSubject": {
           "id": "did:example:456",
           "name": "Alice Johnson",
           "degree": {
             "type": "BachelorDegree",
             "name": "Bachelor of Science in Mechanical Engineering"
           }
         },
         "proof": {
           "type": "Ed25519Signature2020",
           "created": "2023-06-01T19:23:24Z",
           "verificationMethod": "did:example:123#key-1",
           "proofPurpose": "assertionMethod",
           "proofValue": "z58DAdkxz7A..."
         }
       }],
       "proof": {
         "type": "Ed25519Signature2020",
         "created": "2023-06-02T12:00:00Z",
         "verificationMethod": "did:example:456#key-1",
         "proofPurpose": "authentication",
         "proofValue": "z6MkhaXgBZD...",
         "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
         "domain": "example.com"
       }
     }'
```

**レスポンス例:**

```json
{
  "verified": true
}
```