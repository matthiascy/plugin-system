use interface::MinFunc;
use libloading::Library;

pub fn run_plugin(path: &str) -> Result<(), libloading::Error> {
    unsafe {
        log::info!("Loading plugin from {}", path);
        let lib = Library::new(path)?;

        log::info!("With static variable:");
        let min_static = lib.get::<*mut MinFunc>(b"with_static\0")?.read();
        log::info!("  min(1, 2) = {}", min_static(1, 2));
        log::info!("  min(2, 1) = {}", min_static(2, 1));
        log::info!("  min(-10, 10) = {}", min_static(-10, 10));

        log::info!("With dynamic extern:");
        let min_extern = lib.get::<MinFunc>(b"plugin_sample_min_dyn\0")?;
        log::info!(" min(1, 2) = {}", min_extern(1, 2));
        log::info!(" min(2, 1) = {}", min_extern(2, 1));
        log::info!(" min(-10, 10) = {}", min_extern(-10, 10));
    }

    Ok(())
}
