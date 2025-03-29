use std::error::Error;
use std::io;
use tokio::net::UnixListener;

async fn main(
    mut plugin_channel_out: iced::futures::channel::mpsc::Sender<crate::Message>,
) -> Result<(), Box<dyn Error>> {
    let (mut app_channel_out, mut plugin_channel_in) = iced::futures::channel::mpsc::channel(100);

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
