
/// Represents a buffer
/// to which data can be written to.
/// 
/// The serialization and deserialization
/// is the same as Java's [`DataOutput`](https://docs.oracle.com/javase/7/docs/api/java/io/DataOutput.html) class.
pub struct Writer(pub Vec<u8>);


impl Writer {
    pub fn new() -> Self {
        return Writer(Vec::new());
    }

    pub fn u64(&mut self, i: u64) {
        self.0.extend([
            (0xff & (i >> 56)) as u8,
            (0xff & (i >> 48)) as u8,
            (0xff & (i >> 40)) as u8,
            (0xff & (i >> 32)) as u8,
            (0xff & (i >> 24)) as u8,
            (0xff & (i >> 16)) as u8,
            (0xff & (i >> 8)) as u8,
            (0xff & i) as u8
        ]);   
    }

    pub fn i64(&mut self, i: i64) {
        self.0.extend([
            (0xff & (i >> 56)) as u8,
            (0xff & (i >> 48)) as u8,
            (0xff & (i >> 40)) as u8,
            (0xff & (i >> 32)) as u8,
            (0xff & (i >> 24)) as u8,
            (0xff & (i >> 16)) as u8,
            (0xff & (i >> 8)) as u8,
            (0xff & i) as u8
        ]);   
    }

    pub fn u32(&mut self, i: u32) {
        self.0.extend([
            (0xff & (i >> 24)) as u8,
            (0xff & (i >> 16)) as u8,
            (0xff & (i >> 8)) as u8,
            (0xff & i) as u8
        ]);   
    }

    pub fn i32(&mut self, i: i32) {
        self.0.extend([
            (0xff & (i >> 24)) as u8,
            (0xff & (i >> 16)) as u8,
            (0xff & (i >> 8)) as u8,
            (0xff & i) as u8
        ]);   
    }

    pub fn i16(&mut self, i: i16) {
        self.0.extend([
            (0xff & (i >> 8)) as u8,
            (0xff & i) as u8
        ]);   
    }

    pub fn u16(&mut self, i: u16) {
        self.0.extend([
            (0xff & (i >> 8)) as u8,
            (0xff & i) as u8
        ]);   
    }

    pub fn u8(&mut self, b: u8) {
        self.0.push(b);
    }

    pub fn bool(&mut self, b: bool) {
        self.u8(if b { 1 } else { 0 });
    }

    /// This method is subtly wrong and
    /// needs to be fixed.
    pub fn str(&mut self, s: &str) {
        self.u16(s.len() as u16);
        self.0.extend(s.as_bytes());
    }

    /// Appends a series of bytes
    /// to this.
    pub fn bytes(&mut self, s: &[u8]) {
        self.0.extend(s);
    }
}

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize // invariant: cannot exceed data.len
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Reader<'a> {
        Self {
            data,
            pos: 0
        }
    }

    pub fn u64(&mut self) -> Option<u64> {
        if self.data.len() - self.pos < 8 {
            None
        } else {
            let value = 
                ((self.data[self.pos] as u64) << 56) +
                ((self.data[self.pos+1] as u64) << 48) +
                ((self.data[self.pos+2] as u64) << 40) +
                ((self.data[self.pos+3] as u64) << 32) +
                ((self.data[self.pos+4] as u64) << 24) +
                ((self.data[self.pos+5] as u64) << 16) +
                ((self.data[self.pos+6] as u64) << 8) +
                ((self.data[self.pos+7] as u64));
            self.pos += 8;
            Some(value)
        }
    }

    pub fn i64(&mut self) -> Option<i64> {
        if self.data.len() - self.pos < 8 {
            None
        } else {
            let value = 
                ((self.data[self.pos] as i64) << 56) +
                ((self.data[self.pos+1] as i64) << 48) +
                ((self.data[self.pos+2] as i64) << 40) +
                ((self.data[self.pos+3] as i64) << 32) +
                ((self.data[self.pos+4] as i64) << 24) +
                ((self.data[self.pos+5] as i64) << 16) +
                ((self.data[self.pos+6] as i64) << 8) +
                ((self.data[self.pos+7] as i64));
            self.pos += 8;
            Some(value)
        }
    }

    pub fn u32(&mut self) -> Option<u32> {
        if self.data.len() - self.pos < 4 {
            None
        } else {
            let value = 
                ((self.data[self.pos] as u32) << 24) +
                ((self.data[self.pos+1] as u32) << 16) +
                ((self.data[self.pos+2] as u32) << 8) +
                ((self.data[self.pos+3] as u32));
            self.pos += 4;
            Some(value)
        }
    }

    pub fn i32(&mut self) -> Option<i32> {
        if self.data.len() - self.pos < 4 {
            None
        } else {
            let value = 
                ((self.data[self.pos] as i32) << 24) +
                ((self.data[self.pos+1] as i32) << 16) +
                ((self.data[self.pos+2] as i32) << 8) +
                ((self.data[self.pos+3] as i32));
            self.pos += 4;
            Some(value)
        }
    }

    pub fn u16(&mut self) -> Option<u16> {
        if self.data.len() - self.pos < 2 {
            None
        } else {
            let value = 
                ((self.data[self.pos] as u16) << 8) +
                ((self.data[self.pos+1] as u16));
            self.pos += 2;
            Some(value)
        }
    }

    pub fn i16(&mut self) -> Option<i16> {
        if self.data.len() - self.pos < 2 {
            None
        } else {
            let value = 
                ((self.data[self.pos] as i16) << 8) +
                ((self.data[self.pos+1] as i16));
            self.pos += 2;
            Some(value)
        }
    }

    pub fn u8(&mut self) -> Option<u8> {
        if self.data.len() == self.pos {
            None
        } else {
            let value = self.data[self.pos];
            self.pos += 1;
            Some(value)
        }
    }

    pub fn peek_u8(&mut self) -> Option<u8> {
        if self.data.len() == self.pos {
            None
        } else {
            let value = self.data[self.pos];
            Some(value)
        }
    }

    pub fn i8(&mut self) -> Option<i8> {
        self.u8().map(|v| v as i8)
    }

    pub fn bool(&mut self) -> Option<bool> {
        self.u8().map(|v| if v == 0 { false } else { true })
    }

    pub fn str(&mut self) -> Option<&str> {
        let len = self.u16();
        len.and_then(|len| {
            self.bytes(len as usize)
        }).map(|bytes| {
            std::str::from_utf8(bytes).unwrap_or("")
        })
    }

    pub fn bytes(&mut self, n: usize) -> Option<&[u8]> {
        if self.data.len() - self.pos < n {
            return None
        }

        Some(&self.data[self.pos..self.pos+n])
    }

    pub fn bytes_remaining(&mut self) -> &[u8] {
        let slice = &self.data[self.pos..];
        self.pos = self.data.len();
        slice
    }
}