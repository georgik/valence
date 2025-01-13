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

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        let byte = value as u8;
        self.write_u8(byte)
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        // Write the u16 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        // Write the u32 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        // Write the i32 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        // Write the u64 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        // Write the i64 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        // Write the f32 value in big-endian order (most significant byte first).
        let bytes = value.to_bits().to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        // Write the f64 value in big-endian order (most significant byte first).
        let bytes = value.to_bits().to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        // Write the u128 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }
}

// Blanket implementation for all types implementing `Write`
impl<T: Write + ?Sized> WriteExt for T {}