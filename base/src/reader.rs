use std::io;

/// Read file contents (thin wrapper)
pub fn read_contents(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}