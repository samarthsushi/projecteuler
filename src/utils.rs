pub trait Integer: Copy + PartialEq + PartialOrd {
    fn zero() -> Self;
    fn rem(self, rhs: Self) -> Self;
}

macro_rules! integer_trait {
    ($t: ty) => {
        impl Integer for $t {
            fn zero() -> Self {
                0
            }
            fn rem(self, rhs: Self) -> Self {
                self % rhs
            }
        }
    };
}

integer_trait!(i8);
integer_trait!(i16);
integer_trait!(i32);
integer_trait!(i64);
integer_trait!(i128);
integer_trait!(u8);
integer_trait!(u16);
integer_trait!(u32);
integer_trait!(u64);
integer_trait!(u128);

pub fn is_prime<T>(x: T, primes: &[T]) -> bool 
where
    T: Integer + Clone
{
    for &p in primes {
        if x.rem(p) == T::zero() { return false; }
    }
    true
}