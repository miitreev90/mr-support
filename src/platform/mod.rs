// Platform abstractions: Windows, macOS, Linux, Android, iOS
pub mod android;
pub mod ios;
pub mod linux;
pub mod macos;
pub mod windows;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "android") {
            Platform::Android
        } else if cfg!(target_os = "ios") {
            Platform::IOS
        } else {
            Platform::Linux
        }
    }
}
