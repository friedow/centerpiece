use {
    interprocess::local_socket::{prelude::*, GenericFilePath, GenericNamespaced, Stream},
    std::io::{prelude::*, BufReader},
};

fn main() -> std::io::Result<()> {
    let name = if GenericNamespaced::is_supported() {
        "example.sock".to_ns_name::<GenericNamespaced>()?
    } else {
        "/tmp/example.sock".to_fs_name::<GenericFilePath>()?
    };

    let mut buffer = String::with_capacity(128);
    let conn = Stream::connect(name)?;
    let mut conn = BufReader::new(conn);
    conn.get_mut().write_all(b"Hello from client!\n")?;
    conn.read_line(&mut buffer)?;
    print!("Server answered: {buffer}");
    Ok(())
}
