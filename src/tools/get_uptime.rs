use windows::Win32::System::SystemInformation::GetSystemTimeAsFileTime;
use std::sync::LazyLock;

static CURRENT_FILETIME: LazyLock<u64> = LazyLock::new(|| {
    let ft = unsafe { GetSystemTimeAsFileTime() };
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
});

pub fn get_socket_uptime(socket_timestamp: i64) -> u64 {
    (*CURRENT_FILETIME - socket_timestamp as u64) / 10_000_000 // Nano-seconds to seconds
}