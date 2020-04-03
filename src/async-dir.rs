use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use async_std::task;

use std::env::args;

fn main() -> io::Result<()> {
    let path = args().nth(1).expect("missing path argument");

    task::block_on(async {
        let mut dir = fs::read_dir(&path).await?;
        while let Some(res) = dir.next().await {
            let entry = res?;
            println!("{}", entry.file_name().to_string_lossy());
        }

        Ok(())
    })
}
