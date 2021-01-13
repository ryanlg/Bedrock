use core::ops::{Deref, DerefMut};
use core::mem::MaybeUninit;
use core::cell::UnsafeCell;

use crate::core::{Core as CoreTrait};
use crate::arch::x86::console::{DualConsole, Color};


pub struct Core {
    pub console: DualConsole,
}

impl Core {
    fn new() -> Self {
        Core {
            console: DualConsole::new(),
        }
    }
}

impl CoreTrait for Core {

    type ConsoleColor = Color;
    type Console = DualConsole;

    fn get_console(&mut self) -> &mut Self::Console {
        &mut self.console
    }

    fn get_global() -> &'static mut Self {
        unsafe { &mut *__CORE_GLOBAL }
    }
}

/// Dummy structure to hold the Core globally.
/// `initialized` indicates whether the core has been initialized yet,
/// similiar to Once.
///
/// The idea came from `lazy_static` and `spin::Once`: use an UnsafeCell
/// to hold a mutable refernece to a flag of whether Core is initialized, and
/// a mutable ref to MaybeUninit because, yes the core will be uninitialized at
/// the beginning, and by implementing Deref adn DerefMut, the core is brought
/// to life.
static mut __CORE_GLOBAL: __Core_Dummy = __Core_Dummy {
    initialized: UnsafeCell::new(false),
    core:        UnsafeCell::new(MaybeUninit::<Core>::uninit()),
};


/// Not thread safe at all
unsafe impl Sync for __Core_Dummy {}

#[allow(non_camel_case_types)]
struct __Core_Dummy {
    initialized: UnsafeCell<bool>,
    core:        UnsafeCell<MaybeUninit<Core>>
}

impl __Core_Dummy {
    fn try_initialize(&self) {
        unsafe {
            if !*self.initialized.get() {

                (*self.core.get()).write(Core::new());
                *self.initialized.get() = true;
            }
        }
    }
}

impl Deref for __Core_Dummy {
    type Target = Core;
    fn deref(&self) -> &Self::Target {
        self.try_initialize();
        unsafe { &*(*self.core.get()).as_ptr() }
    }
}

impl DerefMut for __Core_Dummy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.try_initialize();
        unsafe { &mut *(*self.core.get()).as_mut_ptr() }
    }
}
