//! A reader/writer/seekable type that chains two underlying streams sequentially.
use std::{
    fmt,
    io::{self, Read, Seek, SeekFrom, Write},
};

pub struct ChainedIO<A, B> {
    first: A,
    second: B,
    len_first: u64,
    len_second: u64,
    pos: u64,
}

impl<A, B> ChainedIO<A, B>
where
    A: Seek,
    B: Seek,
{
    pub fn new(mut first: A, mut second: B) -> io::Result<Self> {
        let cur1 = first.seek(SeekFrom::Current(0))?;
        let len_first = first.seek(SeekFrom::End(0))?;
        first.seek(SeekFrom::Start(cur1))?;

        let cur2 = second.seek(SeekFrom::Current(0))?;
        let len_second = second.seek(SeekFrom::End(0))?;
        second.seek(SeekFrom::Start(cur2))?;

        Ok(Self {
            first,
            second,
            len_first,
            len_second,
            pos: 0,
        })
    }

    fn total_len(&self) -> u64 {
        self.len_first + self.len_second
    }
}

impl<A, B> Read for ChainedIO<A, B>
where
    A: Read + Seek,
    B: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.total_len() {
            return Ok(0);
        }

        let mut read = 0;

        if self.pos < self.len_first {
            self.first.seek(SeekFrom::Start(self.pos))?;
            let n = self.first.read(buf)?;
            self.pos += n as u64;
            read += n;

            if n < buf.len() {
                self.second.seek(SeekFrom::Start(0))?;
                let m = self.second.read(&mut buf[n..])?;
                self.pos += m as u64;
                read += m;
            }
        } else {
            let off = self.pos - self.len_first;
            self.second.seek(SeekFrom::Start(off))?;
            let n = self.second.read(buf)?;
            self.pos += n as u64;
            read += n;
        }

        Ok(read)
    }
}

impl<A, B> Write for ChainedIO<A, B>
where
    A: Write + Seek,
    B: Write + Seek,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        if self.pos < self.len_first {
            self.first.seek(SeekFrom::Start(self.pos))?;
            let n = self.first.write(buf)?;
            self.pos += n as u64;
            written += n;

            if n < buf.len() {
                self.second.seek(SeekFrom::Start(0))?;
                let m = self.second.write(&buf[n..])?;
                self.pos += m as u64;
                written += m;
            }
        } else {
            let off = self.pos - self.len_first;
            self.second.seek(SeekFrom::Start(off))?;
            let n = self.second.write(buf)?;
            self.pos += n as u64;
            written += n;
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.first.flush()?;
        self.second.flush()?;
        Ok(())
    }
}

impl<A, B> Seek for ChainedIO<A, B>
where
    A: Seek,
    B: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let new_pos: i128 = match pos {
            SeekFrom::Start(p) => p as i128,
            SeekFrom::End(o) => self.total_len() as i128 + o as i128,
            SeekFrom::Current(o) => self.pos as i128 + o as i128,
        };

        if new_pos < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid seek"));
        }

        self.pos = new_pos as u64;
        Ok(self.pos)
    }
}

impl<A, B> fmt::Debug for ChainedIO<A, B>
where
    A: fmt::Debug,
    B: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChainedIO")
            .field("first", &self.first)
            .field("second", &self.second)
            .field("len_first", &self.len_first)
            .field("len_second", &self.len_second)
            .field("pos", &self.pos)
            .finish()
    }
}

impl<A, B> Default for ChainedIO<A, B>
where
    A: Default + Read + Write + Seek,
    B: Default + Read + Write + Seek,
{
    fn default() -> Self {
        Self {
            first: A::default(),
            second: B::default(),
            len_first: 0,
            len_second: 0,
            pos: 0,
        }
    }
}
