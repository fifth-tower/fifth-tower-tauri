extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ptr::null_mut;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetWindowThreadProcessId;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winuser::{FindWindowExW, SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};

fn set_window_topmost(pid: u32) {
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
        if process_handle.is_null() {
            eprintln!("无法打开进程");
            return;
        }

        let mut hwnd: winapi::shared::windef::HWND = null_mut();
        loop {
            hwnd = FindWindowExW(null_mut(), hwnd, null_mut(), null_mut());
            if hwnd.is_null() {
                break;
            }

            let mut window_pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, &mut window_pid);
            if window_pid == pid {
                SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE);
                println!("窗口已置顶");
                break;
            }
        }
    }
}

fn main() {
    let pid: u32 = 1234; // 替换为目标进程的 PID
    set_window_topmost(pid);
}
