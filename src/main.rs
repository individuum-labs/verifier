use std::str::FromStr;

use ::serde::{Deserialize, Serialize};
use alloy::signers::local::PrivateKeySigner;
use axum::{
    extract::{Json, Query},
    routing::get,
    Router,
};
use ethers::{abi::Token, types::U256, utils::keccak256};
use hex_literal::hex;
use k256::ecdsa::{hazmat::SignPrimitive, RecoveryId, SigningKey};
use p256::pkcs8::DecodePublicKey;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use tlsn_core::proof::{SessionProof, TlsProof};

#[derive(Serialize, Deserialize)]
struct ResultData {
    pub favourite_count: u32,
    pub post_id: u128,
}

#[derive(Serialize, Deserialize)]
struct VerifyResult {
    pub data: ResultData,
    pub signature: String,
}

async fn verify(Json(proof): Json<TlsProof>) -> Json<VerifyResult> {
    proof
        .session
        .verify_with_default_cert_verifier(notary_pubkey())
        .unwrap();

    let SessionProof {
        // The session header that was signed by the Notary is a succinct commitment to the TLS transcript.
        header,
        // This is the session_info, which contains the server_name, that is checked against the
        // certificate chain shared in the TLS handshake.
        session_info,
        ..
    } = proof.session;

    assert_eq!(session_info.server_name.as_str(), "x.com");

    // Verify the substrings proof against the session header.
    //
    // This returns the redacted transcripts
    let (mut sent, mut recv) = proof.substrings.verify(&header).unwrap();

    // Replace the bytes which the Prover chose not to disclose with 'X'
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');
    let data = ResultData {
        favourite_count: 0,
        post_id: 0,
    };

    let data_to_sign = ethers::abi::encode(&[
        Token::Uint(U256::from(data.favourite_count)),
        Token::Uint(U256::from(data.post_id)),
    ]);

    let secp: Secp256k1<secp256k1::All> = Secp256k1::new();
    let secret =
        SecretKey::from_str("19d8b35c7b3e69e9fedd0fed968574e8acd80f1e733ad4420afa8dd32a9b24cd")
            .unwrap();
    let signature = secp
        .sign_ecdsa(
            &Message::from_digest(sha256::Hash::hash(&data_to_sign).to_byte_array()),
            &secret,
        )
        .to_string();

    Json(VerifyResult { data, signature })
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/verify", get(verify));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn notary_pubkey() -> p256::PublicKey {
    let pem_file = std::str::from_utf8(include_bytes!("../notary.pub")).unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}
