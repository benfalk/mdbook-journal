use super::*;

pub trait SectionNumberExtension: Sized {
    fn root(&self) -> Option<Self>;
    fn root_value(&self) -> Option<u32>;
    fn advance_level(&self) -> Self;
    fn increment(&mut self);
}

impl SectionNumberExtension for SectionNumber {
    fn root(&self) -> Option<Self> {
        self.root_value().map(|val| Self::new(vec![val]))
    }

    fn root_value(&self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            Some(self[0])
        }
    }

    fn advance_level(&self) -> Self {
        if self.is_empty() {
            Self::new(vec![0])
        } else {
            let mut data = self.clone();
            data.push(0);
            data
        }
    }

    fn increment(&mut self) {
        if self.is_empty() {
            self.push(1);
        } else {
            *self.last_mut().unwrap() += 1;
        }
    }
}
