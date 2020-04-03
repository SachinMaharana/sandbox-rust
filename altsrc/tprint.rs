// use log::{info, LevelFilter};
// use std::env::args;
// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// // use tokio::stream::StreamExt;

// const LEN: usize = 16 * 1024; // 16 Kb
// #[tokio::main]
// async fn main() {
//     femme::start(LevelFilter::Trace).unwrap();

//     let path = args().nth(1).expect("Missing Path Argument");

//     if let Err(e) = print(&path).await {
//         info!("Error:{}", e);
//     }
// }

// async fn print(path: &str) -> io::Result<()> {
//     let mut file = File::open(path).await?;
//     let mut stdout = io::stdout();
//     let mut buf = vec![0u8; LEN];
//     loop {
//         let n = file.read(&mut buf).await?;
//         if n == 0 {
//             stdout.flush().await?;
//             return Ok(());
//         }

//         stdout.write_all(&buf[..n]).await?
//     }
// }

use tokio::time::{self, Duration};

// async fn some_async_work() {
//     // do work
// }

async fn do_stuff_async() {
    // async work
}

async fn more_async_work() {
    // more here
}

#[tokio::main]
async fn main() {
    let mut delay = time::delay_for(Duration::from_secs(5));
    let mut lol = time::delay_for(Duration::from_secs(6));
    tokio::select! {
        _ = &mut delay => {
            println!("do_stuff_async() completed first")
        }
        _ = &mut lol => {
            println!("more_async_work() completed first")
        }
    };
}
