use std::{ffi::c_void, net::{Ipv4Addr, Ipv6Addr}, ptr};

use windows::Win32::{Foundation::NO_ERROR, NetworkManagement::IpHelper::{GetExtendedUdpTable, MIB_UDP6ROW_OWNER_MODULE, MIB_UDP6TABLE_OWNER_MODULE, MIB_UDPROW_OWNER_MODULE, MIB_UDPTABLE_OWNER_MODULE, UDP_TABLE_OWNER_MODULE}, System::{ProcessStatus::GetProcessImageFileNameW, Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}}};

use crate::tools::{nt_to_dos::to_dos_path, socket::{Socket, IPV4_ULAF, IPV6_ULAF}};

pub fn get_udp_sockets() -> Vec<Socket> {
    let mut udp_sockets: Vec<Socket> = Vec::new();

    let mut table_size = 0;
    let result = unsafe {
        GetExtendedUdpTable(
            Some(ptr::null_mut() as *mut c_void),
            &mut table_size,
            false,
            IPV4_ULAF,
            UDP_TABLE_OWNER_MODULE,
            0,
        )
    };

    if result != NO_ERROR.0 {
        if result == windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER.0 {
            let mut buffer = vec![0u8; table_size as usize];
            let final_result = unsafe {
                GetExtendedUdpTable(
                    Some(buffer.as_mut_ptr() as *mut c_void),
                    &mut table_size,
                    false,
                    IPV4_ULAF,
                    UDP_TABLE_OWNER_MODULE,
                    0,
                )
            };

            if final_result == windows::Win32::Foundation::NO_ERROR.0 {
                let table_ptr = buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_MODULE;
                let num_entries = unsafe { (*table_ptr).dwNumEntries };
                let row_ptr = unsafe { &((*table_ptr).table) as *const MIB_UDPROW_OWNER_MODULE };

                for i in 0..num_entries {
                    let row = unsafe { &(*row_ptr.add(i as usize)) };
                    match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, row.dwOwningPid) } {
                        Ok(handle) => {
                            let mut path_buffer= [0u16; 260];
                            let length =  unsafe { GetProcessImageFileNameW(handle, &mut path_buffer) };
                            if length > 0 {
                                let path = String::from_utf16_lossy(&path_buffer[..length as usize]);
                                udp_sockets.push(
                                    Socket {
                                        process_name: path.split("\\").last().unwrap_or("unknown").trim().to_string(),
                                        pid: row.dwOwningPid,
                                        port: u16::from_be((row.dwLocalPort & 0xFFFF) as u16),
                                        protocol: "UDP",
                                        remote_addr: None,
                                        local_addr: Ipv4Addr::from(row.dwLocalAddr.to_be()).to_string(),
                                        remote_port: None,
                                        state: "-".to_string(),
                                        executable_path: to_dos_path(&path)
                                    }
                                );
                            }
                            unsafe { windows::Win32::Foundation::CloseHandle(handle).ok(); }
                        }
                        Err(_) => {}
                    } 
                }
            } 
        }
    }
    udp_sockets
}

pub fn get_udp_sockets_ipv6() -> Vec<Socket> {
    let mut udp_sockets: Vec<Socket> = Vec::new();

    let mut table_size = 0;
    let result = unsafe {
        GetExtendedUdpTable(
            Some(ptr::null_mut() as *mut c_void),
            &mut table_size,
            false,
            IPV6_ULAF,
            UDP_TABLE_OWNER_MODULE,
            0,
        )
    };

    if result != NO_ERROR.0 {
        if result == windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER.0 {
            let mut buffer = vec![0u8; table_size as usize];
            let final_result = unsafe {
                GetExtendedUdpTable(
                    Some(buffer.as_mut_ptr() as *mut c_void),
                    &mut table_size,
                    false,
                    IPV6_ULAF,
                    UDP_TABLE_OWNER_MODULE,
                    0,
                )
            };

            if final_result == windows::Win32::Foundation::NO_ERROR.0 {
                let table_ptr = buffer.as_ptr() as *const MIB_UDP6TABLE_OWNER_MODULE;
                let num_entries = unsafe { (*table_ptr).dwNumEntries };
                let row_ptr = unsafe { &((*table_ptr).table) as *const MIB_UDP6ROW_OWNER_MODULE };

                for i in 0..num_entries {
                    let row = unsafe { &(*row_ptr.add(i as usize)) };
                    match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, row.dwOwningPid) } {
                        Ok(handle) => {
                            let mut path_buffer= [0u16; 260];
                            let length =  unsafe { GetProcessImageFileNameW(handle, &mut path_buffer) };
                            if length > 0 {
                                let path = String::from_utf16_lossy(&path_buffer[..length as usize]);
                                udp_sockets.push(
                                    Socket {
                                        process_name: path.split("\\").last().unwrap_or("unknown").trim().to_string(),
                                        pid: row.dwOwningPid,
                                        port: u16::from_be((row.dwLocalPort & 0xFFFF) as u16),
                                        protocol: "UDP",
                                        remote_addr: None,
                                        local_addr: Ipv6Addr::from(row.ucLocalAddr).to_string(),
                                        remote_port: None,
                                        state: "-".to_string(),
                                        executable_path: to_dos_path(&path)
                                    }
                                );
                            }
                            unsafe { windows::Win32::Foundation::CloseHandle(handle).ok(); }
                        }
                        Err(_) => {}
                    } 
                }
            } 
        }
    }
    udp_sockets
}