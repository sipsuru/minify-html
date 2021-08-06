use crate::gen::codepoints::Lookup;

mod bang;
mod comment;
mod content;
mod element;
mod instruction;
mod script;
mod style;
mod textarea;

pub struct Code<'c> {
    code: &'c [u8],
    next: usize,
}

#[derive(Copy, Clone)]
pub struct Checkpoint(usize);

impl<'c> Code<'c> {
    pub fn new(code: &[u8]) -> Code {
        Code {
            code,
            next: 0,
        }
    }

    pub fn str(&self) -> &[u8] {
        &self.code[self.next..]
    }

    pub fn take_checkpoint(&self) -> Checkpoint {
        Checkpoint(self.next)
    }

    pub fn restore_checkpoint(&mut self, cp: Checkpoint) -> () {
        self.next = cp.0;
    }

    pub fn at_end(&self) -> bool {
        self.next == self.code.len()
    }

    pub fn shift_if_next(&mut self, c: u8) -> bool {
        if self.code.get(self.next).filter(|&&n| n == c).is_some() {
            self.next += 1;
            true
        } else {
            false
        }
    }

    pub fn shift_if_next_in_lookup(&mut self, lookup: &'static Lookup) -> Option<u8> {
        let c = self.code.get(self.next).filter(|&&n| lookup[n]).map(|&c| c);
        if c.is_some() {
            self.next += 1;
        };
        c
    }

    pub fn shift_if_next_seq(&mut self, seq: &'static [u8]) -> bool {
        if self.code.get(self.next..self.next + seq.len()).filter(|&n| n == seq).is_some() {
            self.next += seq.len();
            true
        } else {
            false
        }
    }

    pub fn shift(&mut self, n: usize) -> () {
        self.next += n;
    }

    pub fn copy_and_shift(&mut self, n: usize) -> Vec<u8> {
        let str = self.code[self.next..self.next + n].to_vec();
        self.next += n;
        str
    }

    pub fn copy_and_shift_while_in_lookup(&mut self, lookup: &'static Lookup) -> Vec<u8> {
        let mut len = 0;
        loop {
            match self.code.get(self.next + len) {
                Some(&c) if lookup[c] => len += 1,
                _ => break,
            };
        };
        self.copy_and_shift(len)
    }

    pub fn copy_and_shift_while_not_in_lookup(&mut self, lookup: &'static Lookup) -> Vec<u8> {
        let mut len = 0;
        loop {
            match self.code.get(self.next + len) {
                Some(&c) if !lookup[c] => len += 1,
                _ => break,
            };
        };
        self.copy_and_shift(len)
    }

    // Returns the last character matched.
    pub fn shift_while_in_lookup(&mut self, lookup: &'static Lookup) -> Option<u8> {
        let mut last: Option<u8> = None;
        loop {
            match self.code.get(self.next) {
                Some(&c) if lookup[c] => {
                    self.next += 1;
                    last = Some(c);
                }
                _ => break,
            };
        };
        last
    }

    pub fn get(&self, i: usize) -> Option<u8> {
        self.code.get(self.next + i).map(|&c| c)
    }

    pub fn rem(&self) -> usize {
        self.code.len() - self.next
    }
}