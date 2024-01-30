use super::{
    guard::Guard,
    solve,
    types::{Arrow, File, Type},
};

pub struct Index<'a> {
    file: &'a File,
    tree: &'a Guard,
    hats: &'a solve::File,
    indx: Vec<Type>,
}

impl<'a> Index<'a> {
    pub fn make(file: &'a File, tree: &'a Guard, hats: &'a solve::File) -> Self {
        let mut rslt = Self {
            file,
            tree,
            hats,
            indx: Vec::new(),
        };
        rslt.search(file.assoc[file.terms.origin], false);
        rslt
    }

    fn search(&mut self, kind: Type, push: bool) {
        if !self.indx.contains(&kind) {
            match kind {
                Type::Basic(_) => self.indx.push(kind),
                Type::Arrow(arrow) => {
                    if push {
                        self.indx.push(kind);
                    }
                    self.search(self.file.arrow[arrow].0, self.tree.guard[arrow].0);
                    self.search(self.file.arrow[arrow].1, self.tree.guard[arrow].1);
                }
            }
        }
    }

    fn print0(indx: usize) -> String {
        const N: usize = 26;
        const XS: [char; N] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        if indx >= N {
            Self::print0(indx / N - 1) + &XS[indx % N].to_string()
        } else {
            XS[indx].to_string()
        }
    }

    fn print1(&self, kind: Type, hats: usize, left: bool) -> String {
        let mut text = if let Some(indx) = self.indx.iter().position(|&k| k == kind) {
            Self::print0(indx)
        } else if let Type::Arrow(arrow) = kind {
            let text = self.print2(arrow);
            if hats == 0 && left {
                format!("({})", text)
            } else {
                text
            }
        } else {
            unreachable!()
        };
        for _ in 0..hats {
            text = format!("[{}]", text);
        }
        text
    }

    fn print2(&self, arrow: Arrow) -> String {
        let (kl, kr) = self.file.arrow[arrow];
        let (hl, hr) = self.hats.vtype[arrow];
        format!(
            "{} => {}",
            self.print1(kl, self.hats.solve[hl] as usize, true),
            self.print1(kr, self.hats.solve[hr] as usize, false),
        )
    }

    pub fn prints(&self) -> String {
        let origin = self.file.terms.origin;
        let origin = self.print1(
            self.file.assoc[origin],
            self.hats.solve[self.hats.vterm[origin]] as usize,
            false,
        );
        self.indx
            .iter()
            .enumerate()
            .filter_map(|(indx, &kind)| match kind {
                Type::Basic(_) => None,
                Type::Arrow(arrow) => Some(format!(
                    "{} := {}\n",
                    Self::print0(indx),
                    self.print2(arrow),
                )),
            })
            .fold(String::new(), |acc, xs| acc + &xs)
            + &origin
    }
}
