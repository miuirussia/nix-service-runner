use std::fs::{metadata, remove_file};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "service";

    match UnixStream::connect(socket_path) {
        Ok(mut sock) => {
          sock
            .write(b"CONNECT")
            .context("Failed to write onto the unix stream")?;

          sock
            .shutdown(std::net::Shutdown::Write)
            .context("Could not shutdown writing on the stream")?;

          let mut response = String::new();

          sock
            .read_to_string(&mut response)
            .context("Failed to read the unix stream")?;

          println!("We reveived message: {}", response);
        }
        Err(_) => {
            if metadata(socket_path).is_ok() {
                println!("A socket is already present. Deleting...");
                remove_file(socket_path).with_context(|| {
                    format!("could not delete previous socket at {:?}", socket_path)
                })?;
            }

            let unix_listener =
                UnixListener::bind(socket_path).context("Could not create the unix socket")?;

            loop {
                let (unix_stream, _socket_address) = unix_listener
                    .accept()
                    .context("Failed at accepting a connection on the unix listener")?;
                handle_stream(unix_stream)?;
            }
        }
    }

    Ok(())
}

fn handle_stream(mut unix_stream: UnixStream) -> anyhow::Result<()> {
    let mut message = String::new();

    unix_stream
        .read_to_string(&mut message)
        .context("Failed at reading the unix stream")?;

    println!("We received message: {}", message);

    unix_stream
        .write(b"DONE")
        .context("Failed to write onto the unix stream")?;

    Ok(())
}
