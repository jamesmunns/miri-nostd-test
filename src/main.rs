#![no_std]
#![no_main]
#![feature(start)]

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    0
}
