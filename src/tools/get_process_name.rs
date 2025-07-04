use windows::{
    Win32::System::Diagnostics::ToolHelp::*,
    Win32::Foundation::*,
};

pub fn get_process_name_toolhelp(pid: u32) -> Option<String> {
    unsafe {
        let snapshot = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
            Ok(handle) => handle,
            Err(_) => return None,
        };

        if snapshot == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut entry = PROCESSENTRY32W { 
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32, 
            ..Default::default() 
        };

        if Process32FirstW(snapshot, &mut entry).is_err() {
            let _ = CloseHandle(snapshot);
            return None;
        }

        loop {
            if entry.th32ProcessID == pid {
                let _ = CloseHandle(snapshot);
                let len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                return Some(String::from_utf16_lossy(&entry.szExeFile[..len]).to_string());
            }
            if Process32NextW(snapshot, &mut entry).is_err() {
                break;
            }
        }

        let _ = CloseHandle(snapshot);
        None
    }
}