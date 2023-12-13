#[shuttle_runtime::main]
async fn start() -> shuttle_rocket::ShuttleRocket {
    Ok(mxf_api::main().await.into())
}
