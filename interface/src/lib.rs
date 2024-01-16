/// The interface that must be shared between the plugin and the main binary.
pub type MinFunc = unsafe extern "C" fn(i32, i32) -> i32;
