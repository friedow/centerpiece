use std::error::Error;
use std::io;
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = UnixListener::bind("/tmp/centerpiece").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");

                // Wait for the socket to be readable
                stream.readable().await?;

                let mut buf = Vec::with_capacity(4096);

                // Try to read data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                match stream.try_read_buf(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("read {} bytes", n);
                        println!("{}", String::from_utf8_lossy(&buf));
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
            Err(e) => { /* connection failed */ }
        }
    }
    Ok(())
}
