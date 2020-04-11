
struct CC {
    address: String,
    port: u16,
}

impl Drop for CC {
    fn drop(&mut self) {
        println!("Closing Port {}:{}", self.address, self.port);
    }
}

impl CC {
    fn create(address: &str, port: u16) ->CC {
        println!("Opening Port {}:{}", address, port);
        CC{
            address: address.to_string(),
            port: port,
        }
    }

    fn send(&self, msg: &str) {
        println!("Sent to {}:{} the message {}", self.address, self.port, msg);
    }
}

fn main() {
    let channel = CC::create("usb4", 897);
    channel.send("Message 1");
    {
        let channel = CC::create("eth1", 1000);
        channel.send("Message 2");
    }
    channel.send("Message 3");
}
