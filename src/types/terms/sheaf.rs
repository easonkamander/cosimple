use super::graph::{Abstr, Apply, File, Ident, Term};
use typed_index_collections::TiVec;

pub struct Sheaf<T> {
    idents: TiVec<Ident, T>,
    abstrs: TiVec<Abstr, T>,
    applys: TiVec<Apply, T>,
}

impl<T> core::ops::Index<Term> for Sheaf<T> {
    type Output = T;

    fn index(&self, index: Term) -> &Self::Output {
        match index {
            Term::Ident(idx) => &self.idents[idx],
            Term::Abstr(idx) => &self.abstrs[idx],
            Term::Apply(idx) => &self.applys[idx],
        }
    }
}

impl<T> core::ops::IndexMut<Term> for Sheaf<T> {
    fn index_mut(&mut self, index: Term) -> &mut Self::Output {
        match index {
            Term::Ident(idx) => &mut self.idents[idx],
            Term::Abstr(idx) => &mut self.abstrs[idx],
            Term::Apply(idx) => &mut self.applys[idx],
        }
    }
}

impl<'a> From<&'a File> for Sheaf<Term> {
    fn from(value: &'a File) -> Self {
        Self {
            idents: value
                .idents
                .iter_enumerated()
                .map(|(idx, _)| Term::Ident(idx))
                .collect(),
            abstrs: value
                .abstrs
                .iter_enumerated()
                .map(|(idx, _)| Term::Abstr(idx))
                .collect(),
            applys: value
                .applys
                .iter_enumerated()
                .map(|(idx, _)| Term::Apply(idx))
                .collect(),
        }
    }
}

impl<T> Sheaf<T> {
    pub fn fmap<U, F: FnMut(T) -> U>(self, mut f: F) -> Sheaf<U> {
        Sheaf {
            idents: self.idents.into_iter().map(&mut f).collect(),
            abstrs: self.abstrs.into_iter().map(&mut f).collect(),
            applys: self.applys.into_iter().map(&mut f).collect(),
        }
    }
}
