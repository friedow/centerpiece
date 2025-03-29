use anyhow::Context;
use std::error::Error;
use std::io;
use tokio::net::UnixListener;

async fn main(
    mut plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
) -> anyhow::Result<(), Box<dyn Error>> {
    println!("RUNNING!");
    let listener = UnixListener::bind("/tmp/centerpiece").unwrap();
    println!("LAUNCHING!");
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

                        plugin_channel_out
                            .try_send(crate::Message::Show)
                            .context(format!("Failed to send message to show application.",))?;
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

pub fn spawn() -> iced::Subscription<crate::Message> {
    iced::Subscription::run(|| {
        iced::stream::channel(100, |plugin_channel_out| async move {
            let main_loop_result = main(plugin_channel_out).await;
            if let Err(error) = main_loop_result {
                log::error!(
                    target: "ipc_server",
                    "{:?}", error,
                );
                panic!();
            }

            #[allow(clippy::never_loop)]
            loop {
                unreachable!();
            }
        })
    })
}
