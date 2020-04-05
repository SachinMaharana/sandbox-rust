use async_std::task;
use std::time::Instant;
use log::{info, LevelFilter};

// Fetch the HTML contents of a web page.
async fn get(url: &str) -> String {
    // surf::get(url).recv_string().await.unwrap()
    attohttpc::get(url).send().unwrap().text().unwrap()
}

fn main() {
    femme::start(LevelFilter::Trace).unwrap();

    task::block_on(async {
        let start = Instant::now();
        let mut tasks = Vec::new();

        // Fetch the list of contributors for the first 40 minor Rust releases.
        for i in 0..40 {
            let url = format!("https://thanks.rust-lang.org/rust/1.{}.0/", i);

            // Spawn a task fetching the list.
            tasks.push(task::spawn(async move {
                let html = get(&url).await;

                // Display the number of contributors to this Rust release.
                for line in html.lines() {
                    if line.contains("individuals") {
                        info!("{}", line.trim());
                    }
                }
            }))
        }

        // Wait for all tasks to complete.
        for t in tasks {
            t.await;
        }

        // Display elapsed time.
        dbg!(start.elapsed());
    });
}