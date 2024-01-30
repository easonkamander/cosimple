use super::{shape, terms, terms::Sheaf};
use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Basic(usize);

#[derive(Clone, Copy, PartialEq, Eq, From, Into)]
pub struct Arrow(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Basic(Basic),
    Arrow(Arrow),
}

pub struct File {
    pub terms: terms::File,
    pub basic: TiVec<Basic, ()>,
    pub arrow: TiVec<Arrow, (Type, Type)>,
    pub assoc: Sheaf<Type>,
}

impl From<shape::File> for File {
    fn from(value: shape::File) -> Self {
        let count = TiVec::from_iter(value.types.iter().scan(
            (Basic::from(0), Arrow::from(0)),
            |(basic, arrow), shape| match shape {
                shape::Type::Basic() => {
                    let kind = Type::Basic(*basic);
                    *basic = Basic::from(usize::from(*basic) + 1);
                    Some(Some(kind))
                }
                shape::Type::Clone(_) => Some(None),
                shape::Type::Arrow(_, _) => {
                    let kind = Type::Arrow(*arrow);
                    *arrow = Arrow::from(usize::from(*arrow) + 1);
                    Some(Some(kind))
                }
            },
        ));
        let chase = TiVec::from_iter(value.types.iter_enumerated().map(|(name, _)| {
            let name = value.chase(name);
            count[name].unwrap()
        }));
        let basic = TiVec::from_iter(value.types.iter().filter_map(|shape| {
            if let shape::Type::Basic() = *shape {
                Some(())
            } else {
                None
            }
        }));
        let arrow = TiVec::from_iter(value.types.iter().filter_map(|shape| {
            if let shape::Type::Arrow(lhs, rhs) = *shape {
                Some((chase[lhs], chase[rhs]))
            } else {
                None
            }
        }));
        Self {
            terms: value.terms,
            basic,
            arrow,
            assoc: value.assoc.fmap(|i| chase[i]),
        }
    }
}
