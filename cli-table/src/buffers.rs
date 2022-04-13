use std::io::{Result, Write};

use termcolor::{Buffer, BufferWriter, ColorSpec, WriteColor};

pub struct Buffers<'a> {
    pub writer: &'a BufferWriter,
    buffers: Vec<Buffer>,
    current_buffer: Option<Buffer>,
}

impl<'a> Buffers<'a> {
    pub fn new(writer: &'a BufferWriter) -> Self {
        Buffers {
            writer,
            buffers: Vec::new(),
            current_buffer: None,
        }
    }

    pub fn push(&mut self, buffer: Buffer) -> Result<()> {
        if let Some(mut current_buffer) = self.current_buffer.take() {
            current_buffer.reset()?;
            self.buffers.push(current_buffer);
        }

        self.buffers.push(buffer);

        Ok(())
    }

    pub fn append(&mut self, other: &mut Vec<Buffer>) -> Result<()> {
        if let Some(mut current_buffer) = self.current_buffer.take() {
            current_buffer.reset()?;
            self.buffers.push(current_buffer);
        }

        self.buffers.append(other);

        Ok(())
    }

    pub fn into_vec(self) -> Result<Vec<Buffer>> {
        let mut buffers = self.buffers;

        if let Some(mut buffer) = self.current_buffer {
            buffer.reset()?;
            buffers.push(buffer);
        }

        Ok(buffers)
    }
}

impl<'a> Write for Buffers<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(ref mut current_buffer) = self.current_buffer {
            current_buffer.write(buf)
        } else {
            let mut new_buffer = self.writer.buffer();
            let num_bytes = new_buffer.write(buf)?;
            self.current_buffer = Some(new_buffer);

            Ok(num_bytes)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(ref mut current_buffer) = self.current_buffer {
            current_buffer.flush()
        } else {
            Ok(())
        }
    }
}

impl<'a> WriteColor for Buffers<'a> {
    fn supports_color(&self) -> bool {
        match self.current_buffer {
            Some(ref buffer) => buffer.supports_color(),
            None => self.writer.buffer().supports_color(),
        }
    }

    fn set_color(&mut self, color: &ColorSpec) -> Result<()> {
        if let Some(ref mut current_buffer) = self.current_buffer {
            current_buffer.set_color(color)
        } else {
            let mut new_buffer = self.writer.buffer();
            new_buffer.set_color(color)?;
            self.current_buffer = Some(new_buffer);

            Ok(())
        }
    }

    fn reset(&mut self) -> Result<()> {
        if let Some(ref mut current_buffer) = self.current_buffer {
            current_buffer.reset()
        } else {
            Ok(())
        }
    }

    fn is_synchronous(&self) -> bool {
        if let Some(ref buffer) = self.current_buffer {
            buffer.is_synchronous()
        } else {
            self.writer.buffer().is_synchronous()
        }
    }
}
