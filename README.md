## SD-JWT-VC

### 始めに
まだ全然仕様を理解していないので暫定的に必要な機能だけ実装してみます。後々に仕様に準拠するよう改修する予定です。

### 1. システム概要

本システムは、draft-ietf-oauth-sd-jwt-vc-04に完全準拠したVerifiable Credentials (VC) システムを実装します。システムは3つの主要コンポーネント（Issuer、Holder、Verifier）で構成され、各コンポーネントは独立したマイクロサービスとして実装されます。SD-JWT-VCの仕様は[draft-ietf-oauth-sd-jwt-vc-04](https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/)に従う予定です。

### 2. フォルダ構成

```
vc-system/
├── Cargo.lock
├── Cargo.toml
├── README.md
├── docs
│   └── API.md
├── keys
│   └── keys.json
├── src
│   ├── holder
│   │   ├── api.rs
│   │   ├── error.rs
│   │   ├── holder.rs
│   │   ├── mod.rs
│   │   └── storage.rs
│   ├── issuer
│   │   ├── api.rs
│   │   ├── error.rs
│   │   ├── issuer.rs
│   │   ├── mod.rs
│   │   └── schema.rs
│   ├── main.rs
│   ├── models
│   │   ├── credential.rs
│   │   ├── mod.rs
│   │   ├── presentation.rs
│   │   ├── schema.rs
│   │   └── sd_jwt.rs
│   ├── utils
│   │   ├── crypto.rs
│   │   ├── error.rs
│   │   ├── json_ld.rs
│   │   ├── key_manager.rs
│   │   ├── mod.rs
│   │   └── sd_jwt.rs
│   └── verifier
│       ├── api.rs
│       ├── error.rs
│       ├── mod.rs
│       └── verifier.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   │   ├── build
│   │   ├── deps
│   │   ├── examples
│   │   ├── incremental
│   │   ├── vc-system
│   │   └── vc-system.d
│   └── tmp
└── tests
    ├── holder_tests.rs
    ├── issuer_tests.rs
    └── verifier_tests.rs
```

### 3. API設計

#### 3.1 Issuer Service API

1. `POST /credentials`
   - 説明: 新しいVerifiable Credentialを発行します。
   - リクエストボディ:
     ```json
     {
       "@context": ["https://www.w3.org/2018/credentials/v1", ...],
       "type": ["VerifiableCredential", ...],
       "issuer": "did:example:123",
       "issuanceDate": "2023-08-01T12:00:00Z",
       "credentialSubject": {
         "id": "did:example:456",
         "...": "..."
       }
     }
     ```
   - レスポンス:
     ```json
     {
       "@context": ["https://www.w3.org/2018/credentials/v1", ...],
       "type": ["VerifiableCredential", ...],
       "issuer": "did:example:123",
       "issuanceDate": "2023-08-01T12:00:00Z",
       "credentialSubject": {
         "id": "did:example:456",
         "...": "..."
       },
       "proof": {
         "type": "Ed25519Signature2020",
         "created": "2023-08-01T12:00:00Z",
         "verificationMethod": "did:example:123#key-1",
         "proofPurpose": "assertionMethod",
         "proofValue": "z58DAdkxz7A..."
       }
     }
     ```

2. `GET /issuer`
   - 説明: Issuerのメタデータを取得します。
   - レスポンス:
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

#### 3.2 Holder Service API

1. `POST /credentials`
   - 説明: Verifiable Credentialを保存します。
   - リクエストボディ: Issuerから受け取ったVerifiable Credential
   - レスポンス:
     ```json
     {
       "id": "urn:uuid:c410e9a6-d6e8-4c9d-8c6d-9a9d9acb3b87",
       "status": "stored"
     }
     ```

2. `GET /credentials`
   - 説明: 保存されているCredentialのリストを取得します。
   - レスポンス:
     ```json
     {
       "credentials": [
         {
           "id": "urn:uuid:c410e9a6-d6e8-4c9d-8c6d-9a9d9acb3b87",
           "type": ["VerifiableCredential", "UniversityDegreeCredential"],
           "issuer": "did:example:123",
           "issuanceDate": "2023-08-01T12:00:00Z"
         },
         ...
       ]
     }
     ```

3. `POST /presentations`
   - 説明: Verifiable Presentationを作成します。
   - リクエストボディ:
     ```json
     {
       "credentialIds": ["urn:uuid:c410e9a6-d6e8-4c9d-8c6d-9a9d9acb3b87"],
       "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
       "domain": "example.com"
     }
     ```
   - レスポンス: 作成されたVerifiable Presentation

#### 3.3 Verifier Service API

1. `POST /verify/credential`
   - 説明: Verifiable Credentialを検証します。
   - リクエストボディ: 検証するVerifiable Credential
   - レスポンス:
     ```json
     {
       "verified": true,
       "results": [
         {
           "proof": {
             "verified": true,
             "type": "Ed25519Signature2020"
           }
         }
       ]
     }
     ```

2. `POST /verify/presentation`
   - 説明: Verifiable Presentationを検証します。
   - リクエストボディ: 検証するVerifiable Presentation
   - レスポンス:
     ```json
     {
       "verified": true,
       "results": [
         {
           "proof": {
             "verified": true,
             "type": "Ed25519Signature2020"
           }
         },
         {
           "credential": {
             "verified": true,
             "type": ["VerifiableCredential", "UniversityDegreeCredential"]
           }
         }
       ]
     }
     ```

### 4. データモデル

すべてのデータモデルは、W3C Verifiable Credentials Data Model 1.1に厳密に準拠します。

#### 4.1 Verifiable Credential

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.edu/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "https://example.edu/issuers/14",
  "issuanceDate": "2010-01-01T19:23:24Z",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-08-01T12:00:00Z",
    "verificationMethod": "https://example.edu/issuers/14#key-1",
    "proofPurpose": "assertionMethod",
    "proofValue": "z58DAdkxz7A..."
  }
}
```

#### 4.2 Verifiable Presentation

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1"
  ],
  "type": ["VerifiablePresentation"],
  "verifiableCredential": [{
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "https://example.edu/issuers/14",
    "issuanceDate": "2010-01-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    },
    "proof": {
      "type": "Ed25519Signature2020",
      "created": "2023-08-01T12:00:00Z",
      "verificationMethod": "https://example.edu/issuers/14#key-1",
      "proofPurpose": "assertionMethod",
      "proofValue": "z58DAdkxz7A..."
    }
  }],
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-08-02T12:00:00Z",
    "verificationMethod": "did:example:ebfeb1f712ebc6f1c276e12ec21#key-1",
    "proofPurpose": "authentication",
    "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
    "domain": "example.com",
    "proofValue": "z6MkhaXgBZD..."
  }
}
```

### 5. セキュリティ考慮事項

1. すべての通信にTLS/HTTPSを使用
2. 適切な認証・認可メカニズムの実装（JWT、OAuth2）
3. 入力バリデーションの徹底
4. クロスサイトスクリプティング（XSS）対策
5. クロスサイトリクエストフォージェリ（CSRF）対策
6. レート制限の実装
7. 安全な鍵管理システムの実装

### 6. 実装手順

1. 共通ライブラリの実装
2. 各マイクロサービス（Issuer、Holder、Verifier）の基本構造実装
3. データモデルの実装
4. APIエンドポイントの実装
5. 暗号化・署名機能の実装
6. データベース連携（Holder Service）
7. テストの実装（単体テスト、統合テスト、E2Eテスト）
8. セキュリティ強化
9. ドキュメンテーション
10. パフォーマンス最適化