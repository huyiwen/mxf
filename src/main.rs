use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn start(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_rocket::ShuttleRocket {
    Ok(mxf_api::main(secret_store).await.into())
}
