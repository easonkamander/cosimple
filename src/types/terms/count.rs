use super::parse;
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

impl Default for Term {
    fn default() -> Self {
        Term::Ident(Ident(usize::MAX))
    }
}

#[derive(Default)]
pub struct File {
    pub idents: TiVec<Ident, Abstr>,
    pub abstrs: TiVec<Abstr, (usize, Term)>,
    pub applys: TiVec<Apply, (Term, Term)>,
    pub origin: Term,
}

impl File {
    fn insert(
        &mut self,
        world: &parse::File,
        scope: &mut Vec<(parse::Abstr, Abstr)>,
        entry: parse::Term,
    ) -> Result<Term, parse::Ident> {
        match entry {
            parse::Term::Ident(idx) => {
                let binding = scope.iter().rev().find(|&&(old, _)| {
                    let (arg, _) = world.abstrs[old];
                    world.idents[arg] == world.idents[idx]
                });
                if let Some(&(_, new)) = binding {
                    self.abstrs[new].0 += 1;
                    let ident = self.idents.push_and_get_key(new);
                    Ok(Term::Ident(ident))
                } else {
                    Err(idx)
                }
            }
            parse::Term::Abstr(idx) => {
                let value = (0, Term::default());
                let abstr = self.abstrs.push_and_get_key(value);
                scope.push((idx, abstr));
                let (_, val) = world.abstrs[idx];
                self.abstrs[abstr].1 = self.insert(world, scope, val)?;
                scope.pop();
                Ok(Term::Abstr(abstr))
            }
            parse::Term::Apply(idx) => {
                let value = (Term::default(), Term::default());
                let apply = self.applys.push_and_get_key(value);
                let (lhs, rhs) = world.applys[idx];
                self.applys[apply].0 = self.insert(world, scope, lhs)?;
                self.applys[apply].1 = self.insert(world, scope, rhs)?;
                Ok(Term::Apply(apply))
            }
        }
    }
}

impl TryFrom<&parse::File> for File {
    type Error = String;

    fn try_from(value: &parse::File) -> Result<Self, Self::Error> {
        let mut outer = Self::default();
        match outer.insert(value, &mut Vec::new(), value.origin) {
            Ok(entry) => {
                outer.origin = entry;
                Ok(outer)
            }
            Err(ident) => {
                let ident = String::from_utf8_lossy(&value.idents[ident]);
                Err(ident.into_owned())
            }
        }
    }
}
