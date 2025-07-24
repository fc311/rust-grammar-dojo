use std::io::Write;

pub fn log_message<W: Write>(message: &str, writer: &mut W) -> () {
    writeln!(writer, "{}", message).expect("Failed to write to writer");
}
