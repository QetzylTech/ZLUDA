use std::{
    ffi::{c_void, CStr},
    mem, ptr,
};

use std::os::windows::io::AsRawHandle;
use winapi::{
    shared::minwindef::{FARPROC, HMODULE},
    um::debugapi::OutputDebugStringA,
    um::libloaderapi::{GetProcAddress, LoadLibraryW},
};

pub(crate) const LIBCUDA_DEFAULT_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";
const LOAD_LIBRARY_NO_REDIRECT: &'static [u8] = b"ZludaLoadLibraryW_NoRedirect\0";
const GET_PROC_ADDRESS_NO_REDIRECT: &'static [u8] = b"ZludaGetProcAddress_NoRedirect\0";
lazy_static! {
    static ref PLATFORM_LIBRARY: PlatformLibrary = unsafe { PlatformLibrary::new() };
}

#[allow(non_snake_case)]
struct PlatformLibrary {
    LoadLibraryW: unsafe extern "system" fn(*const u16) -> HMODULE,
    GetProcAddress: unsafe extern "system" fn(hModule: HMODULE, lpProcName: *const u8) -> FARPROC,
}

impl PlatformLibrary {
    #[allow(non_snake_case)]
    unsafe fn new() -> Self {
        let (LoadLibraryW, GetProcAddress) = match Self::get_detourer_module() {
            None => (
                LoadLibraryW as unsafe extern "system" fn(*const u16) -> HMODULE,
                mem::transmute(
                    GetProcAddress
                        as unsafe extern "system" fn(
                            hModule: HMODULE,
                            lpProcName: *const i8,
                        ) -> FARPROC,
                ),
            ),
            Some(zluda_with) => (
                mem::transmute(GetProcAddress(
                    zluda_with,
                    LOAD_LIBRARY_NO_REDIRECT.as_ptr() as _,
                )),
                mem::transmute(GetProcAddress(
                    zluda_with,
                    GET_PROC_ADDRESS_NO_REDIRECT.as_ptr() as _,
                )),
            ),
        };
        PlatformLibrary {
            LoadLibraryW,
            GetProcAddress,
        }
    }

    unsafe fn get_detourer_module() -> Option<HMODULE> {
        let mut module = ptr::null_mut();
        loop {
            module = detours_sys::DetourEnumerateModules(module);
            if module == ptr::null_mut() {
                break;
            }
            let payload = GetProcAddress(module as _, b"ZLUDA_REDIRECT\0".as_ptr() as _);
            if payload != ptr::null_mut() {
                return Some(module as _);
            }
        }
        None
    }
}

pub unsafe fn load_library(libcuda_path: &str) -> *mut c_void {
    let libcuda_path_uf16 = libcuda_path
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    (PLATFORM_LIBRARY.LoadLibraryW)(libcuda_path_uf16.as_ptr()) as _
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    (PLATFORM_LIBRARY.GetProcAddress)(handle as _, func.as_ptr() as _) as _
}

#[macro_export]
macro_rules! os_log {
    ($format:tt) => {
        {
            use crate::os::__log_impl;
            __log_impl(format!($format));
        }
    };
    ($format:tt, $($obj: expr),+) => {
        {
            use crate::os::__log_impl;
            __log_impl(format!($format, $($obj,)+));
        }
    };
}

pub fn __log_impl(s: String) {
    let log_to_stderr = std::io::stderr().as_raw_handle() != ptr::null_mut();
    if log_to_stderr {
        eprintln!("[ZLUDA_DUMP] {}", s);
    } else {
        let mut win_str = String::with_capacity("[ZLUDA_DUMP] ".len() + s.len() + 2);
        win_str.push_str("[ZLUDA_DUMP] ");
        win_str.push_str(&s);
        win_str.push_str("\n\0");
        unsafe { OutputDebugStringA(win_str.as_ptr() as *const _) };
    }
}

#[cfg(target_arch = "x86")]
pub fn get_thunk(
    original_fn: *const c_void,
    report_fn: unsafe extern "system" fn(*const [u8; 16], usize),
    guid: *const [u8; 16],
    idx: usize,
) -> *const c_void {
    use dynasmrt::{dynasm, DynasmApi};
    let mut ops = dynasmrt::x86::Assembler::new().unwrap();
    let start = ops.offset();
    dynasm!(ops
        ; .arch x86
        ; push idx as i32
        ; push guid as i32
        ; mov eax, report_fn as i32
        ; call eax
        ; mov eax, original_fn as i32
        ; jmp eax
        ; int 3
    );
    let exe_buf = ops.finalize().unwrap();
    let result_fn = exe_buf.ptr(start);
    mem::forget(exe_buf);
    result_fn as *const _
}

//RCX, RDX, R8, R9
#[cfg(target_arch = "x86_64")]
pub fn get_thunk(
    original_fn: *const c_void,
    report_fn: unsafe extern "system" fn(*const [u8; 16], usize),
    guid: *const [u8; 16],
    idx: usize,
) -> *const c_void {
    use dynasmrt::{dynasm, DynasmApi};
    let mut ops = dynasmrt::x86::Assembler::new().unwrap();
    let start = ops.offset();
    dynasm!(ops
        ; .arch x64
        ; mov [rsp+0x20], r9
        ; mov [rsp+0x18], r8
        ; mov [rsp+0x10], rdx
        ; mov [rsp+0x08], rcx
        // 0x20 for shadow space, 0x38 for 7 stack args, aligns to 16 bytes
        ; sub rsp, 0x58
        ; mov rcx, QWORD guid as i64
        ; mov rdx, QWORD idx as i64
        ; mov rax, QWORD report_fn as i64
        ; call rax
        ; mov rax, [rsp+0x58+0x58]
        ; mov [rsp+0x50], rax
        ; mov rax, [rsp+0x50+0x58]
        ; mov [rsp+0x48], rax
        ; mov rax, [rsp+0x48+0x58]
        ; mov [rsp+0x40], rax
        ; mov rax, [rsp+0x40+0x58]
        ; mov [rsp+0x38], rax
        ; mov rax, [rsp+0x38+0x58]
        ; mov [rsp+0x30], rax
        ; mov rax, [rsp+0x30+0x58]
        ; mov [rsp+0x28], rax
        ; mov rax, [rsp+0x28+0x58]
        ; mov [rsp+0x20], rax
        ; mov r9,  [rsp+0x20+0x58]
        ; mov r8,  [rsp+0x18+0x58]
        ; mov rdx, [rsp+0x10+0x58]
        ; mov rcx, [rsp+0x08+0x58]
        ; mov rax, QWORD original_fn as i64
        ; call rax
        ; add rsp, 0x58
        ; ret
        ; int 3
    );
    let exe_buf = ops.finalize().unwrap();
    let result_fn = exe_buf.ptr(start);
    mem::forget(exe_buf);
    result_fn as *const _
}
