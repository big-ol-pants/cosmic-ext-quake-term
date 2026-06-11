use std::io::Write;
use std::path::PathBuf;

pub const TOGGLE_COMMAND: &str = "toggle";

pub fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);

    runtime_dir.join("cosmic-quake-term.sock")
}

pub fn send_toggle() -> std::io::Result<()> {
    let path = socket_path();

    let mut stream = std::os::unix::net::UnixStream::connect(path)?;
    stream.write_all(TOGGLE_COMMAND.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    Ok(())
}
