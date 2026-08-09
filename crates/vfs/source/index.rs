use std::vec;

pub struct Index {
    offsets: Vec<u32>,
}

impl Index {
    pub fn new(text: &str) -> Self {
        u32::try_from(text.len()).expect("too long text");

        let mut offsets: Vec<u32> = vec![0];

        for (i, _) in text.match_indices('\n') {
            offsets.push((i + 1) as u32);
        }

        Self { offsets }
    }

    pub fn size(&self) -> u32 {
        self.offsets.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double() {
        assert_eq!(Index::new("Hello,\r\nworld!").offsets, vec![0, 8]);
    }

    #[test]
    fn nothing() {
        assert_eq!(Index::new("").offsets, vec![0]);
    }

    #[test]
    fn single() {
        assert_eq!(Index::new("Hello, world!").offsets, vec![0]);
    }

    #[test]
    fn triple() {
        assert_eq!(Index::new("Hello\n,\nworld!").offsets, vec![0, 6, 8]);
    }
}
