//http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust

use futures::future::join_all;
// use futures::stream::StreamExt;
use std::collections::HashMap;
use std::time::Instant;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = vec![
        "https://example.com/1.html".to_string(),
        "https://example.com/2.html".to_string(),
        "https://example.com/3.html".to_string(),
    ];
    let mut tasks: Vec<task::JoinHandle<Result<(), ()>>> = vec![];
    let start = Instant::now();
    for path in paths {
        let path = path.clone();
        tasks.push(tokio::spawn(async move {
            match reqwest::get(&path).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                    }
                    Err(_) => println!("ERROR reading {}", path),
                },
                Err(_) => println!("ERROR downloading {}", path),
            }
            Ok(())
        }));
        // join_all(tasks).await;
    }
    println!("Started {} tasks. Waiting...", tasks.len());
    for t in tasks {
        t.await;
    }
    // join_all(tasks).await;

    dbg!(start.elapsed());

    Ok(())
}

async fn _v1() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

async fn _v2() -> Result<(), Box<dyn std::error::Error>> {
    let path = "https://httpbin.org/ip";
    match reqwest::get(path).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("Response: {} bytes from {}", text.len(), path);
            }
            Err(_) => println!("Error Reading {}", path),
        },

        Err(_) => println!("Error Downloading {}", path),
    }
    Ok(())
}
