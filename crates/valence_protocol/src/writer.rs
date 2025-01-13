use core::fmt::{self, Write};

pub trait WriteExt: Write {
    fn write_all(&mut self, buf: &[u8]) -> fmt::Result {
        for &byte in buf {
            self.write_char(byte as char)?;
        }
        Ok(())
    }

    fn write_u8(&mut self, byte: u8) -> fmt::Result {
        // Write a single byte as a UTF-8 character
        if let Some(ch) = core::char::from_u32(byte as u32) {
            self.write_char(ch)
        } else {
            Err(fmt::Error) // Handle invalid UTF-8 byte
        }
    }
}

// Blanket implementation for all types implementing `Write`
impl<T: Write + ?Sized> WriteExt for T {}