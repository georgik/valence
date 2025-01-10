/// The `Write` trait allows writing bytes into a buffer or output stream.
pub trait Write {
    /// Writes a buffer into this writer, returning an error on failure.
    fn write(&mut self, buf: &[u8]) -> Result<(), WriteError>;

    /// Writes a single byte into this writer.
    fn write_u8(&mut self, byte: u8) -> Result<(), WriteError> {
        self.write(&[byte])
    }

    /// Writes all bytes in the buffer into this writer.
    ///
    /// This function continues to write bytes until either all bytes have been written
    /// or an error occurs. It ensures the entire buffer is written, similar to `std::io::Write::write_all`.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), WriteError> {
        while !buf.is_empty() {
            let written = match self.write(buf) {
                Ok(()) => buf.len(),
                Err(e) => return Err(e),
            };
            buf = &buf[written..];
        }
        Ok(())
    }
}

/// Represents an error that can occur while writing.
#[derive(Debug)]
pub enum WriteError {
    /// Indicates the write operation failed because the buffer is full.
    BufferFull,
    /// Represents other errors that may occur.
    Other,
}

impl From<WriteError> for anyhow::Error {
    fn from(err: WriteError) -> Self {
        match err {
            WriteError::BufferFull => anyhow::anyhow!("Buffer is full"),
            WriteError::Other => anyhow::anyhow!("An unknown write error occurred"),
        }
    }
}

/// A simple buffer-based writer.
pub struct Buffer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Buffer<'a> {
    /// Creates a new `Buffer` writer with the provided buffer.
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns the portion of the buffer that has been written to.
    pub fn written(&self) -> &[u8] {
        &self.buf[..self.pos]
    }
}

impl<'a> Write for Buffer<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<(), WriteError> {
        if self.pos + buf.len() > self.buf.len() {
            return Err(WriteError::BufferFull);
        }
        self.buf[self.pos..self.pos + buf.len()].copy_from_slice(buf);
        self.pos += buf.len();
        Ok(())
    }
}

/// Blanket implementation of `Write` for `&mut W` where `W: Write`.
impl<W: Write + ?Sized> Write for &mut W {
    fn write(&mut self, buf: &[u8]) -> Result<(), WriteError> {
        (**self).write(buf)
    }

    fn write_u8(&mut self, byte: u8) -> Result<(), WriteError> {
        (**self).write_u8(byte)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), WriteError> {
        (**self).write_all(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_write() {
        let mut storage = [0u8; 10];
        let mut buffer = Buffer::new(&mut storage);

        buffer.write(&[1, 2, 3]).unwrap();
        assert_eq!(buffer.written(), &[1, 2, 3]);

        buffer.write(&[4, 5]).unwrap();
        assert_eq!(buffer.written(), &[1, 2, 3, 4, 5]);

        assert!(buffer.write(&[6, 7, 8, 9, 10, 11]).is_err());
    }

    #[test]
    fn write_all() {
        let mut storage = [0u8; 10];
        let mut buffer = Buffer::new(&mut storage);

        buffer.write_all(&[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(buffer.written(), &[1, 2, 3, 4, 5]);

        assert!(buffer.write_all(&[6, 7, 8, 9, 10, 11]).is_err());
    }

    #[test]
    fn write_via_mutable_ref() {
        let mut storage = [0u8; 10];
        let mut buffer = Buffer::new(&mut storage);

        let writer: &mut dyn Write = &mut buffer;
        writer.write_all(&[1, 2, 3]).unwrap();

        assert_eq!(buffer.written(), &[1, 2, 3]);
    }
}
