#[shuttle_runtime::main]
async fn start() -> shuttle_rocket::ShuttleRocket {
    Ok(api::main().await.into())
}
