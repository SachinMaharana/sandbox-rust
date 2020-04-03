use async_std::task;
use log::{info, LevelFilter};

async fn say_hi() {
    info!("Hello, Async");
}
fn main() {
    femme::start(LevelFilter::Trace).unwrap();

    task::block_on(say_hi())
}
