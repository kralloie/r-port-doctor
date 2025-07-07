use windows::Win32::System::SystemInformation::GetSystemTimeAsFileTime;

fn get_current_filetime() -> u64 {
    let ft = unsafe { GetSystemTimeAsFileTime() };
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

pub fn get_socket_uptime(socket_timestamp: i64) -> u64 {
    let current_filetime = get_current_filetime();
    (current_filetime - socket_timestamp as u64) / 10_000_000 // Nano-seconds to seconds
}