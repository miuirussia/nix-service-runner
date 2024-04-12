use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    // copy-paste this and don't think about it anymore
    // it will be hidden from there on
    if std::fs::metadata(socket_path).is_ok() {
        println!("A socket is already present. Deleting...");
        std::fs::remove_file(socket_path)
            .with_context(|| format!("could not delete previous socket at {:?}", socket_path))?;
    }

    let unix_listener =
        UnixListener::bind(socket_path).context("Could not create the unix socket")?;

    loop {
        let (mut unix_stream, socket_address) = unix_listener
            .accept()
            .context("Failed at accepting a connection on the unix listener")?;
        handle_stream(unix_stream)?;
    }
}

fn handle_stream(mut unix_stream: UnixStream) -> anyhow::Result<()> {
    let mut message = String::new();
    unix_stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("We received this message: {}\nReplying...", message);

    unix_stream
        .write(b"I hear you!")
        .context("Failed at writing onto the unix stream")?;

    Ok(())
}
