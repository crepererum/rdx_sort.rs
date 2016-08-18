use template::RdxSortTemplate;

use std::cmp;
use std::mem;

macro_rules! impl_rdxsort {
    ($t:ty, $alias:ty, $min:expr, $zero:expr) => {
        impl RdxSortTemplate for $t {
            fn cfg_nbuckets() -> usize {
                cmp::max(<$alias as RdxSortTemplate>::cfg_nbuckets(), 3)
            }

            fn cfg_nrounds() -> usize {
                <$alias as RdxSortTemplate>::cfg_nrounds() + 1
            }

            fn get_bucket(&self, round: usize) -> usize {
                if round < <$alias as RdxSortTemplate>::cfg_nrounds() {
                    let alias = unsafe { mem::transmute::<$t, $alias>(*self) };
                    alias.get_bucket(round)
                } else {
                    if *self == $min {
                        0
                    } else if *self >= $zero {
                        2
                    } else {
                        1
                    }

                }
            }
        }
    }
}

impl_rdxsort!(i8, u8, i8::min_value(), 0i8);
impl_rdxsort!(i16, u16, i16::min_value(), 0i16);
impl_rdxsort!(i32, u32, i32::min_value(), 0i32);
impl_rdxsort!(i64, u64, i64::min_value(), 0i64);
