#[macro_use]
extern crate log;
use actix_http::Error;
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "actix_http=trace");
    env_logger::init();

    let client = awc::Client::new();

    let mut response = client.get("http://www.rust-lang.org/")
                             .header("User-Agent", "Actix-web")   
                             .send()
                             .await?;

    println!("Respomse: {:?}", response);

    let body = response.body().await?;


   println!("Downloaded: {:?} bytes", body.len());
    Ok(())
}