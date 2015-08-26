use std::ops::{Rem, Add};

pub trait Mod<RHS=Self> {
    type Output;
    fn modulo(self, rhs: RHS) -> Self::Output;
}

impl<A, B, C> Mod<B> for A where A: Rem<B, Output=C>, B: Clone, C: Add<B, Output=C> + Default + PartialOrd {
    type Output = C;
    fn modulo(self, rhs: B) -> Self::Output {
        let result = self % rhs.clone();
        if result < Self::Output::default() {
            result + rhs
        } else {
            result
        }
    }
}
