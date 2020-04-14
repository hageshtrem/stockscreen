use tokio::runtime::Runtime;

mod yahoo_provider;

fn main() {
    Runtime::new()
        .unwrap()
        .block_on(yahoo_provider::make_request())
        .unwrap();
}
