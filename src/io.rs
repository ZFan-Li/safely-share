use std::io::{self, Read, Write};

use rand::thread_rng;

use crate::datamanip::{recover, Operator};

pub fn share<R: Read, W: Write>(source: &mut R, output: &mut Vec<W>) -> io::Result<()> {
    let rng = thread_rng();
    let number = output.len();
    let mut operator = Operator::new(rng, number);
    for rb in source.bytes() {
        let b = rb?;
        for (idx, &eb) in operator.disturb(b).iter().enumerate() {
            output[idx].write(&[eb])?;
        }
    }
    Ok(())
}

pub fn read_from_vector<R: Iterator<Item = io::Result<u8>>>(
    source: &mut Vec<R>,
) -> io::Result<Option<Vec<u8>>> {
    let mut buffer = Vec::with_capacity(source.len());
    for r in source.iter_mut() {
        if let Some(rb) = r.next() {
            buffer.push(rb?);
        } else {
            return Ok(None);
        }
    }
    Ok(Some(buffer))
}

pub fn gather<R: Read, W: Write>(source: &mut Vec<R>, output: &mut W) -> io::Result<()> {
    let mut vib = source.into_iter().map(|vr| vr.bytes()).collect();
    loop {
        let maybe_cipher = read_from_vector(&mut vib)?;
        if let Some(cipher) = maybe_cipher {
            if let Some(plaintext) = recover(cipher.into_iter()) {
                output.write(&[plaintext])?;
            }
        } else {
            break Ok(());
        }
    }
}
