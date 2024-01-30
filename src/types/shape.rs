use super::{
    terms,
    terms::{Sheaf, Term},
};
use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Name(usize);

pub enum Type {
    Basic(),
    Clone(Name),
    Arrow(Name, Name),
}

pub struct File {
    pub terms: terms::File,
    pub types: TiVec<Name, Type>,
    pub assoc: Sheaf<Name>,
}

impl File {
    pub fn chase(&self, mut name: Name) -> Name {
        while let Type::Clone(raw) = self.types[name] {
            name = raw;
        }
        name
    }

    fn arrow(&mut self, name: Name) -> (Name, Name) {
        match self.types[self.chase(name)] {
            Type::Basic() => {
                let lhs = self.types.push_and_get_key(Type::Basic());
                let rhs = self.types.push_and_get_key(Type::Basic());
                self.types[name] = Type::Arrow(lhs, rhs);
                (lhs, rhs)
            }
            Type::Clone(_) => unreachable!(),
            Type::Arrow(lhs, rhs) => (lhs, rhs),
        }
    }

    fn unify(&mut self, key: Name, alt: Name) {
        let key = self.chase(key);
        let alt = self.chase(alt);
        if key != alt {
            use core::mem::replace;
            if let Type::Arrow(altl, altr) = replace(&mut self.types[alt], Type::Clone(key)) {
                match self.types[key] {
                    Type::Basic() => self.types[key] = Type::Arrow(altl, altr),
                    Type::Clone(_) => unreachable!(),
                    Type::Arrow(keyl, keyr) => {
                        self.unify(keyl, altl);
                        self.unify(keyr, altr);
                    }
                }
            }
        }
    }

    fn check(&mut self, term: Term, kind: Name) {
        self.assoc[term] = kind;
        match term {
            Term::Ident(idx) => {
                let abstr = Term::Abstr(self.terms.idents[idx]);
                let (lhs, _) = self.arrow(self.assoc[abstr]);
                self.unify(lhs, kind);
            }
            Term::Abstr(idx) => {
                let (_, rec) = self.terms.abstrs[idx];
                let (_, rhs) = self.arrow(kind);
                self.check(rec, rhs);
            }
            Term::Apply(idx) => {
                let (lhs, rhs) = self.terms.applys[idx];
                let basic = self.types.push_and_get_key(Type::Basic());
                self.check(rhs, basic);
                let arrow = self.types.push_and_get_key(Type::Arrow(basic, kind));
                self.check(lhs, arrow);
            }
        }
    }
}

impl From<terms::File> for File {
    fn from(value: terms::File) -> Self {
        let sheaf = Sheaf::from(&value);
        let mut outer = Self {
            terms: value,
            types: TiVec::new(),
            assoc: sheaf.fmap(|_| Name::from(usize::MAX)),
        };
        let intro = outer.types.push_and_get_key(Type::Basic());
        outer.check(outer.terms.origin, intro);
        outer
    }
}
