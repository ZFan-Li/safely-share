pub mod io {
    use std::io::{self, Read, Write};

    use rand::thread_rng;

    use crate::datamanip::{recover, Operator};

    pub fn share<R: Read, W: Write>(source: &mut R, mut output: Vec<W>) -> io::Result<()> {
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

    pub fn read_from_vector<R: Read>(source: &mut Vec<R>) -> io::Result<Option<Vec<u8>>> {
        let mut buffer = Vec::with_capacity(source.len());
        for r in source {
            if let Some(rb) = r.bytes().next() {
                buffer.push(rb?);
            } else {
                return Ok(None);
            }
        }
        Ok(Some(buffer))
    }

    pub fn gather<R: Read, W: Write>(mut source: Vec<R>, output: &mut W) -> io::Result<()> {
        loop {
            let maybe_cipher = read_from_vector(&mut source)?;
            if let Some(cipher) = maybe_cipher {
                if let Some(plaintext) = recover(cipher.into_iter()) {
                    output.write(&[plaintext])?;
                }
            } else {
                break Ok(());
            }
        }
    }
}
