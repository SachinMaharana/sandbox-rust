use std::fs::File;
use std::io::Write;
fn main() -> std::io::Result<()> {
    // println!("HE")
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    Ok(())
}

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"Hello world\n");
    out.flush()
}
