use async_std::fs::File;
use async_std::prelude::*;
use async_std::task;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1).expect("Missing Fie");

    task::block_on(async {
        let mut file = File::open(&path).await?;
        let mut contents = Vec::new();

        file.read_to_end(&mut contents).await?;
        println!("{:?}", contents);

        for i in contents {
            println!("{} {}", i, i as char);
        }

        Ok(())
    })
}
