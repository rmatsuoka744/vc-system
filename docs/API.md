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
    "id": "http://example.edu/credentials/7c5feb33-9ec2-478d-9197-3a27352299f9",
    "type": [
        "VerifiableCredential",
        "UniversityDegreeCredential"
    ],
    "issuer": "did:example:123",
    "issuanceDate": "2024-08-22T08:46:01.482990377+00:00",
    "credentialSubject": {
        "degree": {
            "name": "Bachelor of Science in Mechanical Engineering",
            "type": "BachelorDegree"
        },
        "id": "did:example:456",
        "name": "Alice Johnson"
    },
    "proof": {
        "created": "2024-08-22T08:46:01.483877358+00:00",
        "proofPurpose": "assertionMethod",
        "proofValue": "2x16B1Nv5eDX2LJCnnf287yhQXH2fqhFW2KHRBgMBaNRG4tKTCmHBKMRfkqH6xjpST9uRxMoyuN2HFXDXKkbFYcE",
        "type": "Ed25519Signature2020",
        "verificationMethod": "did:example:123#key-1"
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
        "id": "http://example.edu/credentials/e064a36e-a597-49a8-9845-b5d8a6ac5503",
        "type": [
            "VerifiableCredential",
            "SDJWTCredential"
        ],
        "issuer": "did:example:123",
        "issuanceDate": "2024-08-22T08:45:10.615784222+00:00",
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
            "created": "2024-08-22T08:45:10.616220523+00:00",
            "proofPurpose": "assertionMethod",
            "proofValue": "5kHCjywDSHwa1C38uHVafYG8KnzGxqFKtUjfgwidXVdXVyLB42fppo77p5yfpirEkmVeyJ9dKQbnTXWUJzKFfzUP",
            "type": "Ed25519Signature2020",
            "verificationMethod": "did:example:123#key-1"
        }
    },
    "sd_jwt": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJfc2QiOlsiYmdTRFctRHZENno4eFFZVmYwV3dQclBwcGR4TzFVWFhYY1ZhS1BUVERVbyIsImRCZDJCX1IyTzhRaDZiTmxVYzZsV1Y4a1pBdTYyOGZpQXVrcXZROWxVYTAiXSwiX3NkX2FsZyI6InNoYS0yNTYiLCJkZWdyZWUiOnsibmFtZSI6IkJhY2hlbG9yIG9mIFNjaWVuY2UgaW4gTWVjaGFuaWNhbCBFbmdpbmVlcmluZyIsInR5cGUiOiJCYWNoZWxvckRlZ3JlZSJ9LCJmYW1pbHlfbmFtZSI6IkpvaG5zb24iLCJnaXZlbl9uYW1lIjoiQWxpY2UiLCJpYXQiOjE3MjQzMTYzMTAsImlkIjoiZGlkOmV4YW1wbGU6NDU2IiwiaXNzIjoiZGlkOmV4YW1wbGU6MTIzIiwidmN0IjoiU0RKV1RDcmVkZW50aWFsIn0.lO0soFlC0F5OLkPjJ61oqZ67I9aB0vyjRfnB9Dt7j6kHsBjlxtPu9z_pw2Sk5vpcMvbCi_093uj9AHfucuhECw",
    "disclosures": [
        "ky5mfpycfuIGnZuSqoZ5AQ",
        "JzxTfo8t9qFKA0ey3UxPhQ"
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
        "publicKeyMultibase": "z8yQxpqtQyTP4RpnTtBAUFdAEBjndVWuqveTZ2rNq7C2n"
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
    "id": "http://example.edu/credentials/7c5feb33-9ec2-478d-9197-3a27352299f9",
    "type": [
        "VerifiableCredential",
        "UniversityDegreeCredential"
    ],
    "issuer": "did:example:123",
    "issuanceDate": "2024-08-22T08:46:01.482990377+00:00",
    "credentialSubject": {
        "degree": {
            "name": "Bachelor of Science in Mechanical Engineering",
            "type": "BachelorDegree"
        },
        "id": "did:example:456",
        "name": "Alice Johnson"
    },
    "proof": {
        "created": "2024-08-22T08:46:01.483877358+00:00",
        "proofPurpose": "assertionMethod",
        "proofValue": "2x16B1Nv5eDX2LJCnnf287yhQXH2fqhFW2KHRBgMBaNRG4tKTCmHBKMRfkqH6xjpST9uRxMoyuN2HFXDXKkbFYcE",
        "type": "Ed25519Signature2020",
        "verificationMethod": "did:example:123#key-1"
    }
}'
```

**レスポンス例:**

```json
{
    "id": "08e88f8b-c507-429d-bad5-e04e569b965f",
    "status": "stored"
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
[
    {
        "@context": [
            "https://www.w3.org/2018/credentials/v1",
            "https://www.w3.org/2018/credentials/examples/v1"
        ],
        "id": "http://example.edu/credentials/7c5feb33-9ec2-478d-9197-3a27352299f9",
        "type": [
            "VerifiableCredential",
            "UniversityDegreeCredential"
        ],
        "issuer": "did:example:123",
        "issuanceDate": "2024-08-22T08:46:01.482990377+00:00",
        "credentialSubject": {
            "degree": {
                "name": "Bachelor of Science in Mechanical Engineering",
                "type": "BachelorDegree"
            },
            "id": "did:example:456",
            "name": "Alice Johnson"
        },
        "proof": {
            "created": "2024-08-22T08:46:01.483877358+00:00",
            "proofPurpose": "assertionMethod",
            "proofValue": "2x16B1Nv5eDX2LJCnnf287yhQXH2fqhFW2KHRBgMBaNRG4tKTCmHBKMRfkqH6xjpST9uRxMoyuN2HFXDXKkbFYcE",
            "type": "Ed25519Signature2020",
            "verificationMethod": "did:example:123#key-1"
        }
    }
]
```

### 2.3 プレゼンテーション作成

保存されているクレデンシャルを使用してVerifiable Presentationを作成します。

**エンドポイント:** `POST /holder/presentations`

**リクエスト例:**

```bash
curl -X POST http://localhost:8080/holder/presentations \
     -H "Content-Type: application/json" \
     -d '{
       "verifiableCredential": ["08e88f8b-c507-429d-bad5-e04e569b965f"],
       "domain": "example.com",
       "challenge": "1f44d55f-f161-4938-a659-f8026467f126"
     }'
```

**レスポンス例:**

```json
{
    "@context": [
        "https://www.w3.org/2018/credentials/v1"
    ],
    "type": [
        "VerifiablePresentation"
    ],
    "verifiableCredential": [
        {
            "@context": [
                "https://www.w3.org/2018/credentials/v1",
                "https://www.w3.org/2018/credentials/examples/v1"
            ],
            "id": "http://example.edu/credentials/7c5feb33-9ec2-478d-9197-3a27352299f9",
            "type": [
                "VerifiableCredential",
                "UniversityDegreeCredential"
            ],
            "issuer": "did:example:123",
            "issuanceDate": "2024-08-22T08:46:01.482990377+00:00",
            "credentialSubject": {
                "degree": {
                    "name": "Bachelor of Science in Mechanical Engineering",
                    "type": "BachelorDegree"
                },
                "id": "did:example:456",
                "name": "Alice Johnson"
            },
            "proof": {
                "created": "2024-08-22T08:46:01.483877358+00:00",
                "proofPurpose": "assertionMethod",
                "proofValue": "2x16B1Nv5eDX2LJCnnf287yhQXH2fqhFW2KHRBgMBaNRG4tKTCmHBKMRfkqH6xjpST9uRxMoyuN2HFXDXKkbFYcE",
                "type": "Ed25519Signature2020",
                "verificationMethod": "did:example:123#key-1"
            }
        }
    ],
    "proof": {
        "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
        "created": "2024-08-22T08:51:42.787965619+00:00",
        "domain": "example.com",
        "proofPurpose": "assertionMethod",
        "proofValue": "3hzA6ptHpnmhfksMY7ZE1bhNPBZ84kGCMMwVDJp78jD58zkZ6SzYroUfYDKU3x8WgMdE2pRnjj3qJYKiiFftGWCq",
        "type": "Ed25519Signature2020",
        "verificationMethod": "did:example:123#key-1"
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
    "id": "http://example.edu/credentials/7c5feb33-9ec2-478d-9197-3a27352299f9",
    "type": [
        "VerifiableCredential",
        "UniversityDegreeCredential"
    ],
    "issuer": "did:example:123",
    "issuanceDate": "2024-08-22T08:46:01.482990377+00:00",
    "credentialSubject": {
        "degree": {
            "name": "Bachelor of Science in Mechanical Engineering",
            "type": "BachelorDegree"
        },
        "id": "did:example:456",
        "name": "Alice Johnson"
    },
    "proof": {
        "created": "2024-08-22T08:46:01.483877358+00:00",
        "proofPurpose": "assertionMethod",
        "proofValue": "2x16B1Nv5eDX2LJCnnf287yhQXH2fqhFW2KHRBgMBaNRG4tKTCmHBKMRfkqH6xjpST9uRxMoyuN2HFXDXKkbFYcE",
        "type": "Ed25519Signature2020",
        "verificationMethod": "did:example:123#key-1"
    }
}'
```

**レスポンス例:**

```json
{
    "errors": [],
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
    "@context": [
        "https://www.w3.org/2018/credentials/v1"
    ],
    "type": [
        "VerifiablePresentation"
    ],
    "verifiableCredential": [
        {
            "@context": [
                "https://www.w3.org/2018/credentials/v1",
                "https://www.w3.org/2018/credentials/examples/v1"
            ],
            "id": "http://example.edu/credentials/7c5feb33-9ec2-478d-9197-3a27352299f9",
            "type": [
                "VerifiableCredential",
                "UniversityDegreeCredential"
            ],
            "issuer": "did:example:123",
            "issuanceDate": "2024-08-22T08:46:01.482990377+00:00",
            "credentialSubject": {
                "degree": {
                    "name": "Bachelor of Science in Mechanical Engineering",
                    "type": "BachelorDegree"
                },
                "id": "did:example:456",
                "name": "Alice Johnson"
            },
            "proof": {
                "created": "2024-08-22T08:46:01.483877358+00:00",
                "proofPurpose": "assertionMethod",
                "proofValue": "2x16B1Nv5eDX2LJCnnf287yhQXH2fqhFW2KHRBgMBaNRG4tKTCmHBKMRfkqH6xjpST9uRxMoyuN2HFXDXKkbFYcE",
                "type": "Ed25519Signature2020",
                "verificationMethod": "did:example:123#key-1"
            }
        }
    ],
    "proof": {
        "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
        "created": "2024-08-22T08:51:42.787965619+00:00",
        "domain": "example.com",
        "proofPurpose": "assertionMethod",
        "proofValue": "3hzA6ptHpnmhfksMY7ZE1bhNPBZ84kGCMMwVDJp78jD58zkZ6SzYroUfYDKU3x8WgMdE2pRnjj3qJYKiiFftGWCq",
        "type": "Ed25519Signature2020",
        "verificationMethod": "did:example:123#key-1"
    }
}'
```

**レスポンス例:**

```json
{
    "errors": [],
    "verified": true
}
```