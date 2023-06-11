use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct TestWriter {
    storage: Rc<RefCell<Vec<u8>>>,
}

impl TestWriter {
    // Creating a new `TestWriter` just means packaging an empty `Vec` in all
    // the wrappers.
    //
    pub(crate) fn new() -> Self {
        TestWriter { storage: Rc::new(RefCell::new(Vec::new())) }
    }

    // Once we're done writing to the buffer, we can pull it out of the `Rc` and
    // the `RefCell` and inspect its contents.
    //
    fn into_inner(self) -> Vec<u8> {
        Rc::try_unwrap(self.storage).unwrap().into_inner()
    }

    // It's easier to compare strings than byte vectors.
    //
    pub(crate) fn into_string(self) -> String {
        String::from_utf8(self.into_inner()).unwrap()
    }
}

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.storage.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.storage.borrow_mut().flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer() {
        let mut writer = TestWriter::new();
        writer.write_all(b"Hello, world!").unwrap();
        assert_eq!(writer.into_string(), "Hello, world!");
    }

    #[test]
    fn test_flush() {
        let mut writer = TestWriter::new();
        writer.write_all(b"Hello, world!").unwrap();
        writer.flush().unwrap();
        assert_eq!(writer.into_string(), "Hello, world!");
    }
}