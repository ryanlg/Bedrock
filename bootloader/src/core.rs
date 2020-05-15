use core::marker::PhantomData;

use crate::console::{Console, ConsoleColor};

/* Main structure that holds some info that will get shared around */
#[repr(C)]
pub struct Core<C, CC>
    where CC: ConsoleColor, C:  Console<CC>    {

    pub console: C,

    // Yes, we don't need this, but rustc will complain about ConsoleColor not
    // being used if we don't include it. Marking this as PhantomData
    // to calm the compiler down.
    console_color: PhantomData<CC>,
}

impl<C, CC> Core<C, CC>
    where CC: ConsoleColor, C:  Console<CC> {

    pub fn new(c: C) -> Self {
        Core {
            console: c,
            console_color: PhantomData,
        }
    }
}
