use octocrab::Octocrab;
use octocrab::models::Repository;

async fn authenticate() -> octocrab::Result<()> {
    let app_id = 151884;
    let app_private_key = "3c5b83527e53477797e6d8ce359a60a41bfa832f";
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(app_private_key.as_bytes()).unwrap();

    let octocrab = Octocrab::apps().build()?;
    let _repos: Vec<Repository> = octocrab
        .get("/installation/repositories", None::<&()>)
        .await
        .unwrap();

    Ok(())
}

#[test]
fn test() {
    tokio_test::block_on(authenticate()).unwrap();
}