use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    readable: R,
    num_reads: usize,
    num_bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(readable: R) -> ReadStats<R> {
        ReadStats {
            readable,
            num_reads: 0,
            num_bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.readable
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_through
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.num_reads += 1;
        let n = self.readable.read(buf)?;
        self.num_bytes_through += n;
        Ok(n)
    }
}

pub struct WriteStats<W> {
    writable: W,
    num_writes: usize,
    num_bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(writable: W) -> WriteStats<W> {
        WriteStats {
            writable,
            num_writes: 0,
            num_bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writable
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_through
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.num_writes += 1;
        let n = self.writable.write(buf)?;
        self.num_bytes_through += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.writable.flush()
    }
}
