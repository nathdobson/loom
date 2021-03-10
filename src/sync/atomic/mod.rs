//! Mock implementation of `std::sync::atomic`.

mod atomic;

use self::atomic::Atomic;

mod bool;

pub use self::bool::AtomicBool;

mod int;

pub use self::int::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicI128, AtomicIsize};
pub use self::int::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicU128, AtomicUsize};

mod ptr;

pub use self::ptr::AtomicPtr;

pub use std::sync::atomic::Ordering;

/// Signals the processor that it is entering a busy-wait spin-loop.
pub fn spin_loop_hint() {
    crate::thread::yield_now();
}

/// An atomic fence.
pub fn fence(order: Ordering) {
    crate::rt::fence(order);
}
