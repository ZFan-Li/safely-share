#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::datamanip::{recover, Operator};

    fn split_bytes(
        bytes: impl Iterator<Item = u8>,
        number: usize,
    ) -> impl Iterator<Item = Vec<u8>> {
        let rng = thread_rng();
        let mut operator = Operator::new(rng, number);
        bytes.map(move |b| operator.disturb(b))
    }

    fn merge_bytes<IVB>(split_bytes: IVB) -> impl Iterator<Item = u8>
    where
        IVB: Iterator<Item = Vec<u8>>,
    {
        split_bytes.map(|vb| recover(vb.into_iter()).unwrap())
    }

    #[test]
    fn test_consistency() {
        let expected = String::from("Hello world!");
        let result = merge_bytes(split_bytes(expected.bytes(), 5)).collect();
        assert_eq!(expected, String::from_utf8(result).unwrap());
    }

    #[test]
    fn test_randomized() {
        let source: Vec<u8> = vec![0x33, 0xAA, 0x78];
        let split: Vec<Vec<u8>> = split_bytes(source.clone().into_iter(), 2).collect();
        let vec_1: Vec<u8> = split.iter().map(|v| v[0]).collect();
        let vec_2: Vec<u8> = split.iter().map(|v| v[1]).collect();
        assert_ne!(source, vec_1);
        assert_ne!(source, vec_2);
        assert_ne!(vec_1, vec_2);
        let result: Vec<u8> = merge_bytes(split.into_iter()).collect();
        assert_eq!(source, result);
    }

    #[test]
    fn test_seperately_randomized() {
        let source: Vec<u8> = vec![0x45, 0x89, 0x91];
        let split_1: Vec<u8> = split_bytes(source.clone().into_iter(), 4)
            .map(|v| v[0])
            .collect();
        let split_2: Vec<u8> = split_bytes(source.clone().into_iter(), 4)
            .map(|v| v[0])
            .collect();
        assert_ne!(split_1, split_2);
    }

    #[test]
    fn test_edge_empty_stream() {
        let source: Vec<u8> = vec![];
        let mut result = split_bytes(source.into_iter(), 3);
        assert!(result.next().is_none());
    }

    #[test]
    fn test_edge_incorrect_argument() {
        let source: Vec<u8> = vec![0x32, 0x76];
        let mut piece_0 = split_bytes(source.clone().into_iter(), 0);
        let piece_1 = split_bytes(source.clone().into_iter(), 1);
        assert!(piece_0.all(|v| v.len() == 0));
        let piece_1_vec: Vec<Vec<u8>> = piece_1.collect();
        assert!(piece_1_vec.iter().all(|v| v.len() == 1));
        assert_eq!(
            source,
            piece_1_vec.iter().map(|v| v[0]).collect::<Vec<u8>>()
        );
    }
}
