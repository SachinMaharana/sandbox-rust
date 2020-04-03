use async_std::{fs::File, io, prelude::*, task};
use std::time::Duration;

async fn read_file(path: &str) -> io::Result<String> {
    println!("b");

    let mut file = File::open(path).await?;
    task::sleep(Duration::from_secs(3)).await;

    println!("c");

    task::sleep(Duration::from_secs(5)).await;
    println!("d");

    let mut contents = String::new();
    println!("e");

    file.read_to_string(&mut contents).await?;
    println!("f");

    Ok(contents)
}

fn main() {
    let reader_task = task::spawn(async {
        println!("a");

        let result = read_file("data.csv").await;
        println!("g");

        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e),
        }
    });
    println!("h");
    task::block_on(reader_task);
    println!("i");
}
