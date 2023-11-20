#![no_std]
#![warn(clippy::pedantic)]

pub struct Checked<T>(pub T);

macro_rules! wrapper {
    ($t: ty) => {
        impl core::ops::Add for Checked<$t> {
            type Output = Option<Self>;

            fn add(self, rhs: Self) -> Self::Output {
                self.0.checked_add(rhs.0).map(Self)
            }
        }

        impl core::ops::Sub for Checked<$t> {
            type Output = Option<Self>;

            fn sub(self, rhs: Self) -> Self::Output {
                self.0.checked_sub(rhs.0).map(Self)
            }
        }

        impl core::ops::Mul for Checked<$t> {
            type Output = Option<Self>;

            fn mul(self, rhs: Self) -> Self::Output {
                self.0.checked_mul(rhs.0).map(Self)
            }
        }

        impl core::ops::Div for Checked<$t> {
            type Output = Option<Self>;

            fn div(self, rhs: Self) -> Self::Output {
                self.0.checked_div(rhs.0).map(Self)
            }
        }
    };
}

wrapper!(u8);
wrapper!(u16);
wrapper!(u32);
wrapper!(u64);
wrapper!(u128);

wrapper!(i8);
wrapper!(i16);
wrapper!(i32);
wrapper!(i64);
wrapper!(i128);

#[cfg(test)]
mod tests {
    use crate::Checked;

    #[test]
    fn add_0() {
        (Checked(u8::MAX - 1) + Checked(1)).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_1() {
        (Checked(u8::MAX) + Checked(1)).unwrap();
    }

    #[test]
    fn sub_0() {
        (Checked(1_u8) - Checked(1)).unwrap();
    }

    #[test]
    #[should_panic]
    fn sub_1() {
        (Checked(0_u8) - Checked(1)).unwrap();
    }

    #[test]
    fn mul_0() {
        (Checked(16_u8) * Checked(15)).unwrap();
    }

    #[test]
    #[should_panic]
    fn mul_1() {
        (Checked(16_u8) * Checked(16)).unwrap();
    }

    #[test]
    fn div_0() {
        (Checked(255_u8) / Checked(16_u8)).unwrap();
    }

    #[test]
    #[should_panic]
    fn div_1() {
        (Checked(255_u8) / Checked(0)).unwrap();
    }

    #[test]
    #[should_panic]
    fn div_2() {
        (Checked(i8::MIN) / Checked(-1)).unwrap();
    }
}
