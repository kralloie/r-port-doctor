use windows::Win32::System::SystemInformation::GetSystemTimeAsFileTime;
use std::sync::LazyLock;

static CURRENT_FILETIME: LazyLock<u64> = LazyLock::new(|| {
    let ft = unsafe { GetSystemTimeAsFileTime() };
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
});

pub fn get_socket_uptime(socket_timestamp: i64) -> u64 {
    let now = *CURRENT_FILETIME as i64;
    let duration_100ns = std::cmp::max(now - socket_timestamp, 0);
    (duration_100ns / 10_000_000) as u64
}