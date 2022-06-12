#[cfg(test)]
mod tests {

    use crate::datamanip::datamanip::{merge_bytes, split_bytes};

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
}
