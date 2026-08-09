pub struct Index {
    content_length: u32,
    offsets: Vec<u32>,
}

impl Index {
    pub fn new(text: &str) -> Self {
        let content_length = u32::try_from(text.len()).expect("valid text size");

        let mut offsets: Vec<u32> = vec![0];

        for (i, _) in text.match_indices('\n') {
            offsets.push((i + 1) as u32);
        }

        Self {
            content_length,
            offsets,
        }
    }

    pub fn position(&self, offset: u32) -> (u32, u32) {
        let line_start = self.offsets.partition_point(|&index| index <= offset) - 1;

        (line_start as u32, offset - self.offsets[line_start])
    }

    pub fn range(&self, line_index: u32) -> Option<(u32, u32)> {
        let start = self.offsets.get((line_index) as usize).copied()?;
        let end = self
            .offsets
            .get((line_index + 1) as usize)
            .copied()
            .unwrap_or(self.content_length);

        Some((start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn text_set() -> impl Strategy<Value = String> {
        prop_oneof!["[\n\ra-z]{0,32}", "[\n\ra-zа-я]{0,32}", "(?s).{0,32}",]
    }

    proptest! {
        #[test]
        fn endings(text in text_set()) {
            let index = Index::new(&text);

            let last = index.offsets.len() as u32 - 1;

            prop_assert_eq!(index.range(last).unwrap().1, index.content_length);
        }

        #[test]
        fn lines(text in text_set()) {
            let index = Index::new(&text);

            for line in 0..index.offsets.len() as u32 {
                let (start, end) = index.range(line).unwrap();

                prop_assert!(start <= end);
                prop_assert!(end <= index.content_length);
            }
        }

        #[test]
        fn none(text in text_set(), extra in 0u32..64) {
            let index = Index::new(&text);
            let beyond = index.offsets.len() as u32 + extra;
            prop_assert!(index.range(beyond).is_none());
        }

        #[test]
        fn offsets(text in text_set()) {
            let index = Index::new(&text);

            for window in index.offsets.windows(2) {
                prop_assert!(window[0] < window[1]);
            }
        }

        #[test]
        fn positions(text in text_set(), seed in 0usize..4096) {
            let index = Index::new(&text);

            let offset = (seed % (text.len() + 1)) as u32;

            let (line, column) = index.position(offset);
            let (start, _) = index.range(line).expect("valid range");

            prop_assert_eq!(start + column, offset);
        }
    }
}
