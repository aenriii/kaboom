#[macro_export]
macro_rules! kaboom {() => {unsafe {std::ptr::null_mut::<&str>().write(":3");}};}