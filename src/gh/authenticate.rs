use jsonwebtoken::EncodingKey;
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use tracing::info;

const COCOGITTO_BOT_APP_ID: u64 = 151884;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InstallationToken {
    pub token: String,
}

pub async fn authenticate(
    installation_id: u64,
    repository: &str,
    gh_key: &str,
) -> octocrab::Result<Octocrab> {
    let key = EncodingKey::from_rsa_pem(gh_key.as_bytes())
        .expect("Configured GitHub private key is not a valid PEM-encoded RSA key");

    let token = octocrab::auth::create_jwt(COCOGITTO_BOT_APP_ID.into(), &key).unwrap();
    let temp_client = Octocrab::builder().personal_token(token).build()?;
    let installation = temp_client
        .apps()
        .installation(installation_id.into())
        .await?;
    let mut create_access_token = CreateInstallationAccessToken::default();
    create_access_token.repositories = vec![repository.to_string()];
    let access: InstallationToken = temp_client
        .post(
            installation.access_tokens_url.as_ref().unwrap(),
            Some(&create_access_token),
        )
        .await?;

    let authed_client = octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()?;

    info!(
        "Authentication success for repo {} with installation id : {}",
        repository, installation_id
    );

    Ok(authed_client)
}
