use client::{
    scripts::compress_accounts::compress_accounts,
    tests::intiailize::test_initialize_server,
    utils::{clear, log::init_logger},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    clear();

    match compress_accounts().await {
        // match initialize_merkle().await {
        Ok(s) => s,
        Err(e) => {
            println!("{:#?}", e);
        }
    }

    Ok(())
}
