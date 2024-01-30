use super::count;
use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(Clone, Copy, From, Into)]
pub struct Ident(usize);

#[derive(Clone, Copy, From, Into)]
pub struct Abstr(usize);

#[derive(Clone, Copy, From, Into)]
pub struct Apply(usize);

#[derive(Clone, Copy)]
pub enum Term {
    Ident(Ident),
    Abstr(Abstr),
    Apply(Apply),
}

pub type Zone = core::ops::Range<Ident>;

pub struct File {
    pub idents: TiVec<Ident, Abstr>,
    pub abstrs: TiVec<Abstr, (Zone, Term)>,
    pub applys: TiVec<Apply, (Term, Term)>,
    pub origin: Term,
}

impl File {
    fn insert(
        &mut self,
        world: &count::File,
        scope: &mut TiVec<count::Abstr, Ident>,
        entry: count::Term,
    ) -> Term {
        match entry {
            count::Term::Ident(idx) => {
                let abstr = world.idents[idx];
                let ident = scope[abstr];
                scope[abstr] = Ident::from(usize::from(scope[abstr]) + 1);
                Term::Ident(ident)
            }
            count::Term::Abstr(idx) => {
                let (num, rec) = world.abstrs[idx];
                let start = self.idents.next_key();
                let range = start..Ident::from(usize::from(start) + num);
                let empty = Term::Ident(Ident(usize::MAX));
                let abstr = self.abstrs.push_and_get_key((range, empty));
                for _ in 0..num {
                    self.idents.push(abstr);
                }
                scope[idx] = start;
                self.abstrs[abstr].1 = self.insert(world, scope, rec);
                Term::Abstr(abstr)
            }
            count::Term::Apply(idx) => {
                let (lhs, rhs) = world.applys[idx];
                let empty_lhs = Term::Ident(Ident(usize::MAX));
                let empty_rhs = Term::Ident(Ident(usize::MAX));
                let apply = self.applys.push_and_get_key((empty_lhs, empty_rhs));
                self.applys[apply].0 = self.insert(world, scope, lhs);
                self.applys[apply].1 = self.insert(world, scope, rhs);
                Term::Apply(apply)
            }
        }
    }
}

impl From<&count::File> for File {
    fn from(value: &count::File) -> Self {
        let mut outer = Self {
            idents: TiVec::new(),
            abstrs: TiVec::new(),
            applys: TiVec::new(),
            origin: Term::Ident(Ident(usize::MAX)),
        };
        let mut scope = value.abstrs.iter().map(|_| Ident(usize::MAX)).collect();
        outer.origin = outer.insert(value, &mut scope, value.origin);
        outer
    }
}
