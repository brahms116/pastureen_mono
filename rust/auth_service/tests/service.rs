mod common;

#[tokio::test]
async fn demo() {
    let api = common::get_api().await;
}
