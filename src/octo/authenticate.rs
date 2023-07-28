use crate::model::installation_token::InstallationToken;
use jsonwebtoken::EncodingKey;
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::Octocrab;

pub async fn authenticate(installation_id: u64, repository: &str) -> octocrab::Result<Octocrab> {
    let app_id = 151884;

    let env_key = std::env::var("GITHUB_PRIVATE_KEY").expect("GITHUB_PRIVATE_KEY not set");

    let key = EncodingKey::from_rsa_pem(env_key.as_bytes())
        .expect("Configured GitHub private key is not a valid PEM-encoded RSA key");

    let token = octocrab::auth::create_jwt(app_id.into(), &key).unwrap();

    let temp_client = Octocrab::builder().personal_token(token).build()?;

    let installations = temp_client
        .apps()
        .installations()
        .send()
        .await
        .unwrap()
        .take_items();

    let installation = installations
        .iter()
        .find(|installation| installation.id.0 == installation_id)
        .expect("Installation not found");

    let mut create_access_token = CreateInstallationAccessToken::default();
    create_access_token.repositories = vec![repository.to_string()];

    let access: InstallationToken = temp_client
        .post(
            installation.access_tokens_url.as_ref().unwrap(),
            Some(&create_access_token),
        )
        .await
        .unwrap();

    let authed_client = octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .unwrap();

    info!(
        "Authentication success for repo {} with installation id : {}",
        repository, installation_id
    );

    Ok(authed_client)
}
