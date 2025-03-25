use {
    interprocess::local_socket::{prelude::*, GenericNamespaced, ListenerOptions, Stream},
    std::io::{self, prelude::*, BufReader},
};

fn handle_error(conn: io::Result<Stream>) -> Option<Stream> {
    match conn {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("Incoming connection failed: {e}");
            None
        }
    }
}

fn main() -> std::io::Result<()> {
    let printname = "example.sock";
    let name = printname.to_ns_name::<GenericNamespaced>()?;
    let opts = ListenerOptions::new().name(name);
    let listener = match opts.create_sync() {
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
            // When a program that uses a file-type socket name terminates its socket server
            // without deleting the file, a "corpse socket" remains, which can neither be
            // connected to nor reused by a new listener. Normally, Interprocess takes care of
            // this on affected platforms by deleting the socket file when the listener is
            // dropped. (This is vulnerable to all sorts of races and thus can be disabled.)
            //
            // There are multiple ways this error can be handled, if it occurs, but when the
            // listener only comes from Interprocess, it can be assumed that its previous instance
            // either has crashed or simply hasn't exited yet. In this example, we leave cleanup
            // up to the user, but in a real application, you usually don't want to do that.
            eprintln!(
                "Error: could not start server because the socket file is occupied. Please check
                if {printname} is in use by another process and try again."
            );
            return Err(e);
        }
        x => x?,
    };

    eprintln!("Server running at {printname}");

    let mut buffer = String::with_capacity(128);
    for conn in listener.incoming().filter_map(handle_error) {
        let mut conn = BufReader::new(conn);
        println!("Incoming connection!");
        conn.read_line(&mut buffer)?;
        print!("Client answered: {buffer}");
        conn.get_mut().write_all(b"Hello from server!\n")?;
        buffer.clear();
    }
    Ok(())
}
