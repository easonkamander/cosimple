use super::{
    guard::Guard,
    types,
    types::{Arrow, Sheaf, Term, Type},
};
use minilp::{ComparisonOp, Error, OptimizationDirection, Problem, Solution, Variable};
use typed_index_collections::TiVec;

pub struct File {
    pub solve: Solution,
    pub vterm: Sheaf<Variable>,
    pub vtype: TiVec<Arrow, (Variable, Variable)>,
}

impl File {
    pub fn solve(types: &types::File, guard: &Guard) -> Result<Self, Error> {
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let vterm = Sheaf::from(&types.terms).fmap(|_| problem.add_var(1.0, (0.0, f64::INFINITY)));
        let vtype = TiVec::from_iter(guard.guard.iter().map(|&(lhs, rhs)| {
            (
                problem.add_var(1.0, (lhs as u8 as f64, f64::INFINITY)),
                problem.add_var(1.0, (rhs as u8 as f64, f64::INFINITY)),
            )
        }));
        for (ident, &abstr) in types.terms.idents.iter_enumerated() {
            let ident = Term::Ident(ident);
            let abstr = Term::Abstr(abstr);
            let Type::Arrow(arrow) = types.assoc[abstr] else {
                unreachable!()
            };
            problem.add_constraint(
                &[(vterm[ident], 1.0), (vtype[arrow].0, -1.0)],
                ComparisonOp::Ge,
                0.0,
            );
        }
        for (abstr, &(_, rec)) in types.terms.abstrs.iter_enumerated() {
            let abstr = Term::Abstr(abstr);
            let Type::Arrow(arrow) = types.assoc[abstr] else {
                unreachable!()
            };
            problem.add_constraint(
                &[(vtype[arrow].1, 1.0), (vterm[rec], -1.0)],
                ComparisonOp::Eq,
                0.0,
            );
        }
        for (apply, &(func, body)) in types.terms.applys.iter_enumerated() {
            let apply = Term::Apply(apply);
            let Type::Arrow(arrow) = types.assoc[func] else {
                unreachable!()
            };
            problem.add_constraint(
                &[
                    (vterm[func], 1.0),
                    (vtype[arrow].0, 1.0),
                    (vterm[body], -1.0),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (vterm[func], 1.0),
                    (vtype[arrow].1, 1.0),
                    (vterm[apply], -1.0),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        let solve = problem.solve()?;
        Ok(Self {
            solve,
            vterm,
            vtype,
        })
    }
}
