#[cfg(test)]
mod tests {
    use std::io::{self, Stdout, Write};
    use std::sync::Mutex;
    use unit_tuple_logger::log_message;

    // We'll use a global mutex to capture printed output
    // lazy_static is a dependency crate
    lazy_static::lazy_static! {
        static ref OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    }

    // Helper to capture print output
    fn capture_output() -> String {
        let mut output = OUTPUT.lock().unwrap();
        let result = String::from_utf8(output.clone()).unwrap();
        output.clear();
        result
    }

    // // Mock println! to capture output
    // #[macro_export]
    // macro_rules! println {
    //     ($($arg:tt)*) => {
    //         let mut output = OUTPUT.lock().unwrap();
    //         writeln!(output, $($arg)*).unwrap();
    //     };
    // }

    // Struct to redirect stdout to our buffer
    struct OutputCapture {
        original: Stdout,
    }

    impl OutputCapture {
        fn new() -> Self {
            OutputCapture {
                original: io::stdout(),
            }
        }

        fn start() -> Self {
            // Clear the buffer before capturing
            OUTPUT.lock().unwrap().clear();
            // No need to explicitly set stdout; we'll write to OUTPUT directly
            Self::new()
        }
    }

    /// Implements the `Write` trait for `OutputCapture`, allowing it to capture and store
    /// written bytes in a shared output buffer (`OUTPUT`). The `write` method appends the
    /// provided byte slice to the buffer, while the `flush` method is a no-op that always
    /// returns `Ok(())`.
    impl Write for OutputCapture {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut output = OUTPUT.lock().unwrap();
            output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    // // Set up output capture before each test
    // fn setup_output_capture() {
    //     std::io::set_output_capture(Some(OUTPUT.lock().unwrap().downgrade()));
    // }

    // test assertions
    #[test]
    fn test_log_message_prints_correctly() {
        let mut buffer = OUTPUT.lock().unwrap();
        let message = "Hello, Rust!";
        log_message(message, &mut *buffer);
        let output = capture_output();
        assert_eq!(output.trim(), message);
    }

    #[test]
    fn test_log_message_returns_unit() {
        let mut buffer = Vec::new();
        let result = log_message("Test message", &mut buffer);
        assert_eq!(result, ()); // Check that () is returned
    }

    #[test]
    fn test_log_message_empty_string() {
        let mut buffer = OUTPUT.lock().unwrap();
        log_message("", &mut *buffer);
        let output = capture_output();
        assert_eq!(output.trim(), "");
    }
}
