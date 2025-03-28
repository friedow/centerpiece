use std::error::Error;
use std::io;
use std::thread::sleep_ms;
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer

    loop {
        let stream_res = UnixStream::connect("/tmp/centerpiece").await;
        if let Err(err) = stream_res {
            sleep_ms(1000);
            continue;
        }
        let stream = stream_res.unwrap();

        // Wait for the socket to be writable
        stream.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_write(b"hello world") {
            Ok(n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
