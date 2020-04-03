use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use async_std::task;
use log::{info, LevelFilter};
use std::env::args;

fn main() -> io::Result<()> {
    femme::start(LevelFilter::Trace).unwrap();

    let path = args().nth(1).expect("missing path argument");

    task::block_on(async {
        let file = File::open(&path).await?;
        let mut lines = BufReader::new(file).lines();
        let mut count = 0u64;

        while let Some(line) = lines.next().await {
            line?;
            count += 1;
        }
        info!("The file contains {} lines.", count);

        Ok(())
    })
}
