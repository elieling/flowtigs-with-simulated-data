#[cfg(not(target_os = "linux"))]
pub use dummy_implementation::MemoryMeter;
#[cfg(target_os = "linux")]
pub use implementation::MemoryMeter;

#[cfg(target_os = "linux")]
mod implementation {
    use log::info;
    use self_meter::Meter;
    use std::time::Duration;

    pub struct MemoryMeter {
        meter: Meter,
    }

    impl MemoryMeter {
        pub fn new() -> Self {
            let mut meter = Meter::new(Duration::from_secs(1)).unwrap();
            meter.track_current_thread("main");
            meter.scan().unwrap();
            Self { meter }
        }

        pub fn report(&mut self) {
            self.meter.scan().unwrap();
            info!(
                "Current memory usage: {:.0}MiB",
                self.meter.report().unwrap().memory_rss as f64 / (1024.0 * 1024.0)
            );
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod dummy_implementation {
    use log::info;
    pub struct MemoryMeter;

    impl MemoryMeter {
        pub fn new() -> Self {
            Self
        }

        pub fn report(&mut self) {
            info!("Memory reporting only supported on Linux");
        }
    }
}