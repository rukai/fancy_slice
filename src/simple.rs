use byteorder::{BigEndian, ReadBytesExt};

use std::str;
use std::slice::SliceIndex;

#[derive(Clone, Copy)]
pub struct FancySlice<'a> {
    data: &'a [u8],
}

impl<'a> FancySlice<'a> {
    pub fn new(data: &[u8]) -> FancySlice {
        FancySlice { data }
    }

    pub fn relative_fancy_slice<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> FancySlice {
        FancySlice {
            data: &self.data[range]
        }
    }

    pub fn relative_slice<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> &[u8] {
        &self.data[range]
    }

    pub fn u8(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn i16_be(&self, offset: usize) -> i16 {
        (&self.data[offset..]).read_i16::<BigEndian>().unwrap()
    }

    pub fn u16_be(&self, offset: usize) -> u16 {
        (&self.data[offset..]).read_u16::<BigEndian>().unwrap()
    }

    pub fn i32_be(&self, offset: usize) -> i32 {
        (&self.data[offset..]).read_i32::<BigEndian>().unwrap()
    }

    pub fn u32_be(&self, offset: usize) -> u32 {
        (&self.data[offset..]).read_u32::<BigEndian>().unwrap()
    }

    pub fn f32_be(&self, offset: usize) -> f32 {
        (&self.data[offset..]).read_f32::<BigEndian>().unwrap()
    }

    pub fn str(&self, offset: usize) -> Result<&str, String> {
        let data = &self.data[offset..];
        if let Some(length) = data.iter().position(|x| *x == 0) {
            str::from_utf8(&data[..length]).map_err(|x| format!("{}", x))
        }
        else {
            Err(String::from("String was not terminated"))
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Debug display each byte in hex
    pub fn hex<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> String {
        let data = &self.data[range];
        let mut string = String::new();
        for (i, byte) in data.iter().enumerate() {
            if i != 0 && i % 2 == 0 {
                string.push_str(" ");
            }
            string.push_str(&format!("{:02x}", byte));
        }
        string
    }

    /// Debug display each byte as an ascii if valid otherwise display as '.'
    pub fn ascii<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> String {
        let data = &self.data[range];
        let mut string = String::new();
        for byte in data {
            let ascii = *byte as char;
            if ascii.is_ascii_graphic() {
                string.push(ascii);
            }
            else {
                string.push('.');
            }
        }
        string
    }
}
