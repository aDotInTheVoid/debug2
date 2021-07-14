use crate::{Debug, Formatter, Result};

macro_rules! std_debug {
    ($($t:ty),+) => {
        $(
            impl Debug for $t {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    f.write_debug(self)
                }
            }
        )+
    };
}

std_debug! {
    String, &str, bool, (),
    i8, i16, i32, i64, i128, isize,
    f32, f64
}

// impl<T: Debug> Debug for [T] {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         f.debug_list().entries(self.iter()).finish()
//     }
// }
