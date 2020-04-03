use async_std::future;
use async_std::prelude::*;
#[async_std::main]
async fn main() {
    let a = future::ready(1u8);
    let b = future::ready(2u16);
    let f = a.join(b);
    let g = f.await;
    println!("Hello, {:?}", g);
}
