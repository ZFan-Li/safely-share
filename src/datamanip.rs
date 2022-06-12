pub mod datamanip {
    use rand::{thread_rng, Rng};
    use std::iter;

    pub fn split_bytes(
        bytes: impl Iterator<Item = u8>,
        number: usize,
    ) -> impl Iterator<Item = Vec<u8>> {
        let mut rng = thread_rng();
        bytes.map(move |b| {
            let mut tuple: Vec<u8> = iter::once(b)
                .chain(iter::repeat_with(|| rng.gen()))
                .take(number)
                .collect();
            let core = tuple.iter().skip(1).fold(b, |acc, item| acc ^ item);
            if let Some(item) = tuple.get_mut(0) {
                *item = core
            }
            tuple
        })
    }

    pub fn merge_bytes<IVB>(split_bytes: IVB) -> impl Iterator<Item = u8>
    where
        IVB: Iterator<Item = Vec<u8>>,
    {
        split_bytes.map(|ib| ib.iter().skip(1).fold(ib[0], |acc, item| acc ^ item))
    }
}
