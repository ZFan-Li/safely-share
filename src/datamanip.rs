use rand::Rng;
use std::iter;

pub struct Operator<R: Rng> {
    rng: R,
    number: usize,
}

impl<R: Rng> Operator<R> {
    pub fn disturb(&mut self, plaintext: u8) -> Vec<u8> {
        let mut cipher: Vec<u8> = iter::once(plaintext)
            .chain(iter::repeat_with(|| self.rng.gen::<u8>()))
            .take(self.number)
            .collect();
        let core = cipher
            .iter()
            .skip(1)
            .fold(plaintext, |acc, item| acc ^ item);
        if let Some(first) = cipher.get_mut(0) {
            *first = core;
        }
        cipher
    }
    pub fn new(rng: R, number: usize) -> Self {
        Operator { rng, number }
    }
}

pub fn recover(mut cipher: impl Iterator<Item = u8>) -> Option<u8> {
    if let Some(first) = cipher.next() {
        Some(cipher.fold(first, |acc, item| acc ^ item))
    } else {
        None
    }
}
