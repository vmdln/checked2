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
