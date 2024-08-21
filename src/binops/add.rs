/// This macro implement addition and mixed addition for group and field elements.
#[macro_export]
macro_rules! impl_binops_add {
    // Mixed addition implementation.
    // This allows to implement addition for curve points of different types,
    // that is, in different representations.
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> ::core::ops::Add<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: &'b $rhs) -> $output {
                &self + rhs
            }
        }

        impl<'a> ::core::ops::Add<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                self + &rhs
            }
        }

        impl ::core::ops::Add<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                &self + &rhs
            }
        }
    };

    // Output type is equal to first argument type
    ($lhs:ident, $rhs:ident) => {
        impl_binops_add!($lhs, $rhs, $lhs);

        impl ::core::ops::AddAssign<$rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                *self = &*self + &rhs;
            }
        }

        impl<'b> ::core::ops::AddAssign<&'b $rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self + rhs;
            }
        }
    };
}
