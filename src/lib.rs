use std::ops::{Rem, Add};
/// Trait that describes modulo operation
/// # Examples
///
/// ```¨
/// use modulo::Mod;
/// fn main() {
///     assert_eq!(4, (-2).modulo(6));
///     assert_eq!(2, 9.modulo(7));
/// }
/// ```
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

#[cfg(test)]
mod test {
    use super::Mod;

    #[test]
    fn smaller_than_modulus_works() {
        assert_eq!(3, 3.modulo(9));
        assert_eq!(7, 7.modulo(8));
    }

    #[test]
    fn larger_than_modulus_works() {
        assert_eq!(1, 6.modulo(5));
        assert_eq!(8, 18.modulo(10));
    }

    #[test]
    fn megative_works() {
        assert_eq!(1, (-1).modulo(2));
        assert_eq!(2, (-7).modulo(3));
    }

    #[test]
    fn big_number_works() {
        assert_eq!(1, 101.modulo(10));
    }
}
