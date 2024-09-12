#[cfg(any(
    target_os = "macos", 
    target_os = "linux", 
    target_os = "freebsd", 
    target_os = "openbsd", 
    target_os = "netbsd"
))]
mod linux_bsd_mac;
#[cfg(any(
    target_os = "macos", 
    target_os = "linux", 
    target_os = "freebsd", 
    target_os = "openbsd", 
    target_os = "netbsd"
))]
/// unix based systems
pub use linux_bsd_mac::{Protex , ProtexGuard};

#[cfg(any(target_os = "android", target_os = "ios"))]
mod android_ios;
#[cfg(any(target_os = "android", target_os = "ios"))]
/// mobile platforms
pub use android_ios::{Protex , ProtexGuard};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
/// windows
pub use windows::{Protex , ProtexGaurd};
