// TEMPLATE: hal_outer.rs
#[cfg(feature = "qemu")]
mod platform {}

#[cfg(feature = "bbb")]
mod platform {}
