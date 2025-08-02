use chrono::{Duration, Utc};
use config::auth_config::AuthConfig;
use config::base_config::Config;
use jsonwebtoken::{jwk::JwkSet, Header};
use openssl::rsa::Rsa;
use rest::client::HttpClientBuilder;
use service::auth::{
    google::{GoogleClaims, GoogleOAuth},
    oauth_provider::OAuthProvider,
};

static GOOGLE_ISSUERS: &[&str] = &["https://accounts.google.com", "accounts.google.com"];

async fn fetch_first_kid() -> String {
    let auth_config = AuthConfig::new().unwrap();

    // HTTP 클라이언트를 따로 만들어서 키 목록 확인
    let client = HttpClientBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    let response = client
        .get(auth_config.google_oauth_public_key_url)
        .await
        .inspect(|x| println!("{:?}", x))
        .expect("Failed to fetch Google JWKs");

    let jwk_set: JwkSet = response.json().await.expect("Failed to parse JWK set");
    println!("{:?}", jwk_set);

    let first_key = jwk_set.keys.first().unwrap();
    first_key.common.key_id.to_owned().unwrap()
}

#[tokio::test]
async fn test_google_oauth_creation() {
    let result = GoogleOAuth::new("test_client_id".to_string());
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_valid_verify_token_has_to_success() {
    let kid = fetch_first_kid().await;
    let rsa = Rsa::generate(2048).unwrap();
    // PKCS#1 PEM
    let priv_pem_pkcs1 = rsa.private_key_to_pem().unwrap();
    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some(kid);

    let claims = GoogleClaims {
        iss: GOOGLE_ISSUERS.join(" "),
        aud: "client_id".to_owned(),
        sub: "sub".to_owned(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
        iat: Utc::now().timestamp(),
        email: Some("user@example.com".to_owned()),
        email_verified: Some(true),
        name: Some("Test".to_owned()),
    };
}

#[tokio::test]
#[ignore] // 기본 테스트에서는 제외, `cargo test -- --ignored`로 실행
async fn test_fetch_real_google_jwk() {
    let google_oauth = GoogleOAuth::new("test_client_id".to_string()).unwrap();

    // 실제 Google에서 사용되는 키 ID로 테스트 (구글에서 실제로 사용하는 kid)
    // 이 키들은 주기적으로 로테이션되므로, 테스트가 실패할 수 있습니다.

    // 대신 Google JWK 엔드포인트에서 키 목록을 먼저 가져와서 테스트

    let kid = fetch_first_kid().await;
    let result = google_oauth.fetch_public_key(kid.as_str()).await;
    match result {
        Ok(_decoding_key) => {
            println!("Successfully created DecodingKey");
        }
        Err(_) => {
            panic!("Failed")
        }
    }
}
