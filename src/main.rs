use std::str::FromStr;

use ::serde::{Deserialize, Serialize};
use axum::{extract::Json, routing::post, Router};
use ethers::{abi::Token, types::U256, utils::keccak256};
use ethsign::{KeyFile, Protected, SecretKey};
use hyper::Method;
use p256::pkcs8::DecodePublicKey;
use regex::Regex;
use tlsn_core::proof::{SessionProof, TlsProof};
use tower_http::cors::{Any, CorsLayer};
use verifier::tweet::Root;

#[derive(Serialize, Deserialize)]
struct ResultData {
    pub favourite_count: u32,
    pub post_id: u128,
    pub full_text: String,
}

#[derive(Serialize, Deserialize)]
struct VerifyResult {
    pub data: ResultData,
    pub signature_r: [u8; 32],
    pub signature_s: [u8; 32],
    pub signature_v: u8,
}

#[derive(Serialize, Deserialize)]
struct Proofs {
    pub account: TlsProof,
    pub post: TlsProof,
}

async fn verify(Json(proofs): Json<Proofs>) -> Json<VerifyResult> {
    let proof = proofs.post;
    let account = proofs.account;

    proof
        .session
        .verify_with_default_cert_verifier(notary_pubkey())
        .unwrap();

    account
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
    assert_eq!(
        account.session.session_info.server_name.as_str(),
        "api.x.com"
    );

    // Verify the substrings proof against the session header.
    //
    // This returns the redacted transcripts
    let (mut sent, mut recv) = proof.substrings.verify(&header).unwrap();
    let (mut account_sent, mut account_recv) =
        account.substrings.verify(&account.session.header).unwrap();

    // Replace the bytes which the Prover chose not to disclose with 'X'
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');
    account_sent.set_redacted(b'X');
    account_recv.set_redacted(b'X');
    let position = recv
        .data()
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .unwrap();
    let json_data = serde_json::from_slice::<Root>(&recv.data()[(position + 3)..])
        .unwrap()
        .data;
    let instruction = &json_data
        .threaded_conversation_with_injections_v2
        .instructions[0];
    let entry = &instruction.entries.clone().unwrap()[0];
    let data = ResultData {
        favourite_count: entry
            .clone()
            .content
            .item_content
            .unwrap()
            .tweet_results
            .result
            .legacy
            .favorite_count
            .try_into()
            .unwrap(),
        post_id: u128::from_str(
            &entry
                .clone()
                .content
                .item_content
                .unwrap()
                .tweet_results
                .result
                .legacy
                .conversation_id_str,
        )
        .unwrap(),
        full_text: entry
            .clone()
            .content
            .item_content
            .unwrap()
            .tweet_results
            .result
            .legacy
            .full_text,
    };

    let position = account_recv
        .data()
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .unwrap();

    let regex = Regex::new(r#"(?m)"screen_name":"(\w{1,15})""#).unwrap();

    let string = String::from_utf8(account_recv.data()[(position + 3)..].to_vec()).unwrap();
    let matched = regex.captures(&string).unwrap();

    assert_eq!(
        entry
            .clone()
            .content
            .item_content
            .unwrap()
            .tweet_results
            .result
            .core
            .user_results
            .result
            .legacy
            .screen_name,
        matched[1]
    );

    let data_to_sign = ethers::abi::encode(&[
        Token::Uint(U256::from(data.favourite_count)),
        Token::Uint(U256::from(data.post_id)),
        Token::String(data.full_text.clone()),
    ]);

    let file = std::fs::File::open("./keypair").unwrap();
    let key: KeyFile = serde_json::from_reader(file).unwrap();
    let password: Protected = "aaaaaaaa".into();
    let secret = key.to_secret_key(&password).unwrap();
    let signature = secret.sign(&keccak256(&data_to_sign)).unwrap();

    Json(VerifyResult {
        data,
        signature_r: signature.r,
        signature_s: signature.s,
        signature_v: signature.v,
    })
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/verify",
        post(verify).layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST, Method::GET])
                .allow_headers(Any),
        ),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn notary_pubkey() -> p256::PublicKey {
    let pem_file = std::str::from_utf8(include_bytes!("../notary.pub")).unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}
