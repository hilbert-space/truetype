use std::io::{Read, Seek, SeekFrom};

use Result;

pub trait Tape: Read + Seek + Sized {
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[inline]
    fn peek<T: Value>(&mut self) -> Result<T> {
        self.stay(|tape| Value::read(tape))
    }

    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    #[inline(always)]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }

    fn stay<F, T>(&mut self, mut body: F) -> Result<T> where F: FnMut(&mut Self) -> Result<T> {
        let position = try!(self.position());
        let result = body(self);
        try!(self.jump(position));
        result
    }
}

pub trait Value: Sized {
    fn read<T: Tape>(&mut T) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {
}