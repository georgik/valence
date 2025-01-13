// use core::fmt::{self, Write};
use embedded_io::Write;

pub trait WriteExt: Write {
    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
    //     for &byte in buf {
    //         self.write_char(byte as char)?;
    //     }
    //     Ok(())
    // }

    fn write_u8(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.write(&[byte]);
        Ok(())
    }

    fn write_i8(&mut self, value: i8) -> Result<(), Self::Error> {
        let byte = value as u8;
        self.write_u8(byte)
    }

    fn write_u16(&mut self, value: u16) -> Result<(), Self::Error> {
        // Write the u16 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        // Write the u32 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        // Write the i32 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        // Write the u64 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        // Write the i64 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        // Write the f32 value in big-endian order (most significant byte first).
        let bytes = value.to_bits().to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        // Write the f64 value in big-endian order (most significant byte first).
        let bytes = value.to_bits().to_be_bytes();
        self.write_all(&bytes)
    }

    fn write_u128(&mut self, value: u128) -> Result<(), Self::Error> {
        // Write the u128 value in big-endian order (most significant byte first).
        let bytes = value.to_be_bytes();
        self.write_all(&bytes)
    }
}

// Blanket implementation for all types implementing `Write`
impl<T: Write + ?Sized> WriteExt for T {}

use embedded_io::ErrorType;
use bytes::BytesMut;

// Define the Writer struct
pub struct Writer<'a, B> {
    buf: &'a mut B,
}

impl<'a, B> Writer<'a, B> {
    pub fn new(buf: &'a mut B) -> Self {
        Self { buf }
    }
}

// Implement ErrorType for Writer
impl<'a> ErrorType for Writer<'a, BytesMut> {
    type Error = core::convert::Infallible; // Use an appropriate error type if your implementation may fail
}

// Implement Write for Writer
impl<'a> Write for Writer<'a, BytesMut> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // No-op for `BytesMut`, as it does not need flushing.
        Ok(())
    }
}
