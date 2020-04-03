use std::env::args;
use log::{info, LevelFilter};
use tokio::fs::File;
use tokio::io::{self, BufReader,AsyncBufReadExt};
use tokio::stream::StreamExt;


#[tokio::main]
async fn main() {
    femme::start(LevelFilter::Trace).unwrap(); 

    let path = args().nth(1).expect("Missing Path Argument");

    match get_line(&path).await {
        Ok(lines) => {
            info!("The file contains {} lines.", lines);
        }
        Err(e) => println!("Error:{}", e)
    };
    println!("Hello");
}

async fn get_line(path: &str) -> io::Result<u64> {
    let file = File::open(path).await?;
    let mut lines = BufReader::new(file).lines();
    let mut count:u64 = 0u64;

    while let Some(line) = lines.next().await {
        line?;
        count += 1;

    }
    Ok(count)
}
