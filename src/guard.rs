use super::types::{Arrow, File, Type};
use typed_index_collections::TiVec;

struct LinkIter<'a, T> {
    link: &'a TiVec<T, Option<T>>,
    data: Option<T>,
}

impl<'a> Iterator for LinkIter<'a, Arrow> {
    type Item = Arrow;

    fn next(&mut self) -> Option<Self::Item> {
        let past = self.data;
        if let Some(data) = self.data {
            self.data = self.link[data];
        }
        past
    }
}

pub struct Guard {
    pub guard: TiVec<Arrow, (bool, bool)>,
}

impl<'a> From<&'a File> for Guard {
    fn from(value: &'a File) -> Self {
        let mut guard = TiVec::from_iter(value.arrow.iter().map(|_| (false, false)));
        let mut above = TiVec::from_iter(value.arrow.iter().map(|_| None::<Arrow>));
        for (arrow, &(lhs, rhs)) in value.arrow.iter_enumerated() {
            if let Type::Arrow(lhs) = lhs {
                let mut iter = LinkIter {
                    link: &above,
                    data: Some(arrow),
                };
                if iter.find(|&a| a == lhs).is_some() {
                    guard[arrow].0 = true;
                } else {
                    above[lhs].get_or_insert(arrow);
                }
            }
            if let Type::Arrow(rhs) = rhs {
                let mut iter = LinkIter {
                    link: &above,
                    data: Some(arrow),
                };
                if iter.find(|&a| a == rhs).is_some() {
                    guard[arrow].1 = true;
                } else {
                    above[rhs].get_or_insert(arrow);
                }
            }
        }
        Self { guard }
    }
}
