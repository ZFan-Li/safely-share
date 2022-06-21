#[cfg(test)]
mod datamanip {
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

#[cfg(test)]
mod io {
    use std::{io::Read, io::Write};

    use crate::io::{gather, share};

    struct BytesReader<T: Iterator<Item = u8>>(T);

    impl<T: Iterator<Item = u8>> Read for BytesReader<T> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            for i in 0..buf.len() {
                if let Some(b) = self.0.next() {
                    buf[i] = b;
                } else {
                    return Ok(i);
                }
            }
            Ok(buf.len())
        }
    }

    impl<T: Iterator<Item = u8>> BytesReader<T> {
        pub fn new(bytes: T) -> Self {
            BytesReader(bytes)
        }
    }

    struct BytesWriter(Vec<u8>);

    impl Write for BytesWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.extend(buf.clone().into_iter());
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl BytesWriter {
        pub fn new() -> BytesWriter {
            BytesWriter(Vec::new())
        }
    }

    impl IntoIterator for BytesWriter {
        type Item = u8;

        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    #[test]
    fn test_encoding() {
        let mut br = BytesReader::new([0xFE, 0xE3, 0x98].into_iter());
        let mut bws: Vec<BytesWriter> = std::iter::repeat_with(|| BytesWriter::new())
            .take(3)
            .collect();
        assert!(share(&mut br, &mut bws).is_ok());
        assert!(bws.into_iter().all(|bw| bw.into_iter().count() == 3));
    }

    #[test]
    fn test_decoding() {
        let mut brs: Vec<_> =
            std::iter::repeat_with(|| BytesReader::new([0xFE, 0xE3, 0x98].into_iter()))
                .take(5)
                .collect();
        let mut bw = BytesWriter::new();
        assert!(gather(&mut brs, &mut bw).is_ok());
        assert!(bw.into_iter().count() == 3);
    }

    #[test]
    fn test_consistency() {
        let expected = "Hello world!";
        let mut reader = BytesReader::new(expected.bytes());
        let mut writers: Vec<BytesWriter> = std::iter::repeat_with(|| BytesWriter::new())
            .take(5)
            .collect();
        assert!(share(&mut reader, &mut writers).is_ok());
        let mut readers: Vec<_> = writers
            .into_iter()
            .map(|wr| BytesReader::new(wr.into_iter()))
            .collect();
        let mut writer = BytesWriter::new();
        assert!(gather(&mut readers, &mut writer).is_ok());
        let result = String::from_utf8(writer.into_iter().collect());
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}

#[cfg(test)]
mod opt {
    use std::path::PathBuf;

    use crate::opt::{expand_path, get_length, length_of_number, padded_number};

    #[test]
    fn test_number_length() {
        for i in 0..10 {
            assert_eq!(1, length_of_number(i));
        }
        for i in 10..100 {
            assert_eq!(2, length_of_number(i));
        }
        for i in 100..1000 {
            assert_eq!(3, length_of_number(i));
        }
    }

    #[test]
    fn test_pad_number() {
        assert_eq!("7", padded_number(7, 1));
        assert_eq!("07", padded_number(7, 2));
        assert_eq!("007", padded_number(7, 3));
        assert_eq!("100", padded_number(100, 3));
        assert_eq!("100", padded_number(100, 2));
        assert_eq!("100", padded_number(100, 0));
    }

    #[test]
    fn test_expand_path_lt() {
        let path: Vec<PathBuf> = ["ein", "zwei", "drei"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let result = expand_path(path, 10);
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert_eq!(
            ["ein", "zwei", "drei"]
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>(),
            expanded[..3].to_vec()
        );
        assert_eq!(
            (0..7)
                .map(|n| PathBuf::from(n.to_string()))
                .collect::<Vec<_>>(),
            expanded[3..].to_vec()
        );
    }

    #[test]
    fn test_expand_eq() {
        let path: Vec<PathBuf> = ["ein", "zwei", "drei"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let result = expand_path(path, 3);
        assert!(result.is_ok());
        assert_eq!(
            ["ein", "zwei", "drei"]
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>(),
            result.unwrap()
        );
    }

    #[test]
    fn test_expand_gt() {
        let path: Vec<PathBuf> = ["ein", "zwei", "drei"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        assert!(expand_path(path, 2).is_err());
    }

    #[test]
    fn test_expand_many() {
        let result = expand_path(Vec::new(), 11);
        assert!(result.is_ok());
        let path = result.unwrap();
        let expected: Vec<PathBuf> = (0..=10)
            .map(|n| format!("{:02}", n))
            .map(PathBuf::from)
            .collect();
        assert_eq!(expected, path);
    }

    fn test_resource(filename: &str) -> PathBuf {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut path = PathBuf::from(root_dir);
        path.push(".test-resource");
        path.push(filename);
        path
    }

    #[test]
    fn test_equal_length() {
        let equal_paths: Vec<PathBuf> = ["EQUAL-00", "EQUAL-01", "EQUAL-02"]
            .into_iter()
            .map(test_resource)
            .collect();
        let result = get_length(&equal_paths);
        assert!(result.is_ok());
        assert_eq!(11, result.unwrap());
    }

    #[test]
    fn test_unequal_length() {
        let unequal_paths: Vec<PathBuf> = ["EQUAL-00", "UNEQUAL", "EQUAL-01"]
            .into_iter()
            .map(test_resource)
            .collect();
        let result = get_length(&unequal_paths);
        assert!(result.is_err());
    }

    #[test]
    fn test_preceding_empty() {
        let unequal_paths: Vec<PathBuf> = ["EMPTY", "UNEQUAL"]
            .into_iter()
            .map(test_resource)
            .collect();
        let result = get_length(&unequal_paths);
        assert!(result.is_err());
    }
}
