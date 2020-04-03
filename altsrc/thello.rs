use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
