use interface::MinFunc;

/// First approach: use a static variable to store the function pointer.
#[no_mangle]
pub static with_static: MinFunc = plugin_sample_min;

pub extern "C" fn plugin_sample_min(a: i32, b: i32) -> i32 {
    a.min(b)
}

/// Second approach: the function is directly exposed via dynamic linking,
/// using the Rust ABI.
#[no_mangle]
pub extern "C" fn plugin_sample_min_dyn(a: i32, b: i32) -> i32 {
    a.min(b)
}
