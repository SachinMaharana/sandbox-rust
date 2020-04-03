

use log::{info, LevelFilter};
use std::env::args;
use std::io;
// use tokio::stream::StreamExt;

use  tokio::fs;
#[tokio::main]
async fn main() {
    femme::start(LevelFilter::Trace).unwrap(); 

    let path = args().nth(1).expect("missing path argument");

    match read_dir(&path).await {
        Ok(_) => {
            info!("The file contains ines.");
        }
        Err(e) => println!("Erroror:{}", e)
    }
  

}

async fn read_dir(path: &str) -> io::Result<()>{
    let mut dir = fs::read_dir(path).await?;
    while let Some(entry) = dir.next_entry().await? {
        println!("{:?}", entry.file_name());
    }
    Ok(())
}