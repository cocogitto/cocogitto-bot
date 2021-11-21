use crate::model::installation_token::InstallationToken;
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::Octocrab;

pub async fn authenticate(installation_id: u64, repository: &str) -> octocrab::Result<Octocrab> {
    let app_id = 151884;

    let key = std::env::var("GITHUB_PRIVATE_KEY").expect("GITHUB_PRIVATE_KEY not set");
    let token = octocrab::auth::create_jwt(app_id.into(), key).unwrap();

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let installations = octocrab
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

    let access: InstallationToken = octocrab
        .post(
            installation.access_tokens_url.as_ref().unwrap(),
            Some(&create_access_token),
        )
        .await
        .unwrap();

    let octocrab = octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .unwrap();

    info!(
        "Authentication success for repo {} with installation id : {}",
        repository, installation_id
    );

    Ok(octocrab)
}
