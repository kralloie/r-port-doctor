use std::{collections::HashMap, ffi::{OsStr, OsString}, os::windows::ffi::{OsStrExt, OsStringExt}, sync::LazyLock};

use windows::{core::PCWSTR, Win32::Storage::FileSystem::QueryDosDeviceW};

pub static NT_TO_DOS_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map: HashMap<String, String> = HashMap::new();
    for drive_letter in b'A'..=b'Z' {
        let drive = format!("{}:", drive_letter as char);
        let mut device_path_buf = [0u16; 512];
        let drive_wide: Vec<u16> = OsStr::new(&drive)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let len = unsafe {
            QueryDosDeviceW(
                PCWSTR(drive_wide.as_ptr()),
                Some(&mut device_path_buf)
            )
        };

        if len == 0 {
            continue;
        }

        let mut dos_path = OsString::from_wide(&device_path_buf[..len as usize])
            .to_string_lossy()
            .to_string();
        dos_path = dos_path.trim_end_matches('\0').to_string();
        
        map.insert(dos_path, drive);
    }
    map
});

pub fn to_dos_path(nt_path: &str) -> Option<String> {
    let nt_drive = nt_path.split("\\").take(3).collect::<Vec<_>>().join("\\");
    match NT_TO_DOS_MAP.get(&nt_drive).cloned() {
        Some(dos_path) => {
            Some(nt_path.replacen(nt_drive.as_str(), &dos_path, 1))
        }
        None => {
            None
        }
    }
}