use byteorder::{BigEndian, ReadBytesExt};

use std::str;
use std::ops::{RangeBounds, Bound};
use std::slice::SliceIndex;

#[derive(Clone, Copy)]
pub struct FancySlice<'a> {
    //sender: Sender,
    data:     &'a [u8],
    start:    usize,
    end:      usize,
}

impl<'a> FancySlice<'a> {
    pub fn new(data: &[u8]) -> FancySlice {
        FancySlice {
            //sender: Sender::new(),
            data,
            start: 0,
            end:   data.len(),
        }
    }

    // Only compile with release feature
    // Maybe I want to provide an abstraction around the reader or maybe its up to the user to
    // interpret it.
    //
    // Some thoughts:
    // Out of order due to multithreading is fine as the order doesnt matter
    // Each new child can just clone the reader on.
    //pub fn new_with_receiver(data: &[u8]) -> (FancySlice, Receiver) {
    //    let (sender, receiver) = mspc::thing():
    //    let slice = FancySlice {
    //        sender: Receiver::new(),
    //        data,
    //        start: 0,
    //        end:   data.len(),
    //    }
    //    (slice, receiver)
    //}

    pub fn relative_fancy_slice<T: RangeBounds<usize>>(&self, range: T) -> FancySlice {
        FancySlice {
            data:  &self.data,
            start: self.start + bound(range.start_bound(), 0),
            end:   self.start + bound(range.end_bound(), self.data.len() - self.start),
        }
    }

    /// Requires `debug` feature.
    pub fn absolute_fancy_slice<T: RangeBounds<usize>>(&self, range: T) -> FancySlice {
        FancySlice {
            data:  &self.data,
            start: bound(range.start_bound(), 0),
            end:   bound(range.end_bound(), self.data.len() - self.start),
        }
    }

    pub fn relative_slice<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> &[u8] {
        &self.data[self.start..self.end][range]
    }

    /// Requires `debug` feature.
    pub fn absolute_slice<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> &[u8] {
        &self.data[range]
    }

    pub fn u8(&self, offset: usize) -> u8 {
        self.data[self.start + offset]
    }

    pub fn i16_be(&self, offset: usize) -> i16 {
        (&self.data[self.start + offset..]).read_i16::<BigEndian>().unwrap()
    }

    pub fn u16_be(&self, offset: usize) -> u16 {
        (&self.data[self.start + offset..]).read_u16::<BigEndian>().unwrap()
    }

    pub fn i32_be(&self, offset: usize) -> i32 {
        (&self.data[self.start + offset..]).read_i32::<BigEndian>().unwrap()
    }

    pub fn u32_be(&self, offset: usize) -> u32 {
        (&self.data[self.start + offset..]).read_u32::<BigEndian>().unwrap()
    }

    pub fn f32_be(&self, offset: usize) -> f32 {
        (&self.data[self.start + offset..]).read_f32::<BigEndian>().unwrap()
    }

    pub fn str(&self, offset: usize) -> Result<&str, String> {
        let data = &self.data[self.start + offset..];
        if let Some(length) = data.iter().position(|x| *x == 0) {
            str::from_utf8(&data[..length]).map_err(|x| format!("{}", x))
        }
        else {
            Err(String::from("String was not terminated"))
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Requires `debug` feature.
    /// Returns the offset into the original slice.
    pub fn offset(&self) -> usize {
        self.start
    }

    /// Debug display each byte in hex
    pub fn hex<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> String {
        let data = &self.data[self.start..self.end][range];
        let mut string = String::new();
        for (i, byte) in data.iter().enumerate() {
            if i != 0 && i % 2 == 0 {
                string.push_str(" ");
            }
            string.push_str(&format!("{:02x}", byte));
        }
        string
    }

    /// Debug display each byte as an ascii character if valid, otherwise display as '.'
    pub fn ascii<I: SliceIndex<[u8], Output=[u8]>>(&self, range: I) -> String {
        let data = &self.data[self.start..self.end][range];
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

    /// Requires `debug` feature.
    pub fn find_i8(&self, search_for: i8) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() {
            let value = self.data[address] as i8;
            if value == search_for {
                result.push(address);
            }
        }

        result
    }

    /// Requires `debug` feature.
    pub fn find_u8(&self, search_for: u8) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() {
            let value = self.data[address];
            if value == search_for {
                result.push(address);
            }
        }

        result
    }

    /// Requires `debug` feature.
    pub fn find_i16(&self, search_for: i16) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() - 2 {
            let value = (&self.data[address..]).read_i16::<BigEndian>().unwrap();
            if value == search_for {
                result.push(address);
            }
        }

        result
    }

    /// Requires `debug` feature.
    pub fn find_u16(&self, search_for: u16) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() - 2 {
            let value = (&self.data[address..]).read_u16::<BigEndian>().unwrap();
            if value == search_for {
                result.push(address);
            }
        }

        result
    }

    /// Requires `debug` feature.
    pub fn find_i32(&self, search_for: i32) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() - 4 {
            let value = (&self.data[address..]).read_i32::<BigEndian>().unwrap();
            if value == search_for {
                result.push(address);
            }
        }

        result
    }

    /// Requires `debug` feature.
    pub fn find_u32(&self, search_for: u32) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() - 4 {
            let value = (&self.data[address..]).read_u32::<BigEndian>().unwrap();
            if value == search_for {
                result.push(address);
            }
        }

        result
    }

    /// Requires `debug` feature.
    pub fn find_offset_pointers(&self, search_for_address: usize) -> Vec<usize> {
        let mut result = vec!();

        for address in 0..self.data.len() - 4 {
            let value = (&self.data[address..]).read_u32::<BigEndian>().unwrap();
            if address + value as usize == search_for_address {
                result.push(address);
            }
        }

        result
    }
}

#[derive(Debug)]
pub struct PointerSearchResults {
    pub absolute: Vec<usize>,
    pub relative: Vec<usize>,
}

fn bound(bound: Bound<&usize>, or: usize) -> usize {
    match bound {
        Bound::Included(a) => *a,
        Bound::Excluded(a) => *a, // TODO: +-1 !?!?
        Bound::Unbounded => or
    }
}
