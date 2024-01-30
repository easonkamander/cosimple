use chumsky::prelude::*;
use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(From, Into, Clone, Copy)]
pub struct Ident(usize);

#[derive(From, Into, Clone, Copy)]
pub struct Abstr(usize);

#[derive(From, Into, Clone, Copy)]
pub struct Apply(usize);

#[derive(Clone, Copy)]
pub enum Term {
    Ident(Ident),
    Abstr(Abstr),
    Apply(Apply),
}

pub struct File {
    pub idents: TiVec<Ident, Vec<u8>>,
    pub abstrs: TiVec<Abstr, (Ident, Term)>,
    pub applys: TiVec<Apply, (Term, Term)>,
    pub origin: Term,
}

impl TryFrom<&[u8]> for File {
    type Error = Vec<Simple<u8>>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use core::cell::RefCell;

        let idents = RefCell::new(TiVec::new());
        let abstrs = RefCell::new(TiVec::new());
        let applys = RefCell::new(TiVec::new());

        let total = recursive(|recur| {
            let paren = recur.clone().delimited_by(just('(' as u8), just(')' as u8));

            let ident = text::ident().map(|raw| idents.borrow_mut().push_and_get_key(raw));

            let block = ident.map(Term::Ident).or(paren).padded();

            let apply = block.clone().then(block.repeated()).foldl(|lhs, rhs| {
                let idx = applys.borrow_mut().push_and_get_key((lhs, rhs));
                Term::Apply(idx)
            });

            let arrow = ident.padded().then_ignore(just(['=' as u8, '>' as u8]));

            let abstr = arrow.then(recur).map(|abs| {
                let idx = abstrs.borrow_mut().push_and_get_key(abs);
                Term::Abstr(idx)
            });

            abstr.or(apply)
        });

        let origin = total.then_ignore(end()).parse(value)?;

        Ok(Self {
            idents: idents.into_inner(),
            abstrs: abstrs.into_inner(),
            applys: applys.into_inner(),
            origin,
        })
    }
}
