use cuda_types::*;
use std::{
    ffi::{c_void, CStr},
    fmt::LowerHex,
    mem, ptr, slice,
};

use cuda_base::cuda_derive_display_trait;

pub(crate) trait CudaDisplay {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()>;
}

impl CudaDisplay for cuda_types::CUuuid {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        let guid = self.bytes;
        write!(writer, "{{{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}", guid[3], guid[2], guid[1], guid[0], guid[5], guid[4], guid[7], guid[6], guid[8], guid[9], guid[10], guid[11], guid[12], guid[13], guid[14], guid[15])
    }
}

impl CudaDisplay for cuda_types::CUdevice {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", self.0)
    }
}

impl CudaDisplay for cuda_types::CUdeviceptr {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}

impl CudaDisplay for cuda_types::CUdeviceptr_v1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0 as usize as *const ())
    }
}

impl CudaDisplay for u8 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u16 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for i32 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u32 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u64 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for usize {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for f32 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

pub fn write_handle<T: LowerHex>(
    this: &[T; 64],
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"0x")?;
    for i in (0..64).rev() {
        write!(writer, "{:02x}", this[i])?;
    }
    Ok(())
}

impl CudaDisplay for cuda_types::CUipcMemHandle {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_handle(&self.reserved, writer)
    }
}

impl CudaDisplay for cuda_types::CUipcEventHandle {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_handle(&self.reserved, writer)
    }
}

impl CudaDisplay for cuda_types::CUmemPoolPtrExportData_v1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_handle(&self.reserved, writer)
    }
}

impl CudaDisplay for *mut c_void {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", *self)
    }
}

impl CudaDisplay for *const c_void {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", *self)
    }
}

impl CudaDisplay for *const i8 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if *self == ptr::null_mut() {
            writer.write_all(b"NULL")
        } else {
            write!(
                writer,
                "\"{}\"",
                unsafe { CStr::from_ptr(*self as _) }.to_string_lossy()
            )
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Luid {
    low_part: u32,
    high_part: u32,
}

impl CudaDisplay for *mut i8 {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if fn_name == "cuDeviceGetLuid" && index == 0 {
            let luid_ptr = *self as *mut Luid;
            let luid = unsafe { *luid_ptr };
            write!(writer, "{{{:08X}-{:08X}}}", luid.low_part, luid.high_part)
        } else {
            write!(
                writer,
                "\"{}\"",
                unsafe { CStr::from_ptr(*self as _) }.to_string_lossy()
            )
        }
    }
}

impl CudaDisplay for cuda_types::CUstreamBatchMemOpParams {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        unsafe {
            match self.operation {
                // The below is not a typo, `WAIT_VALUE` and `WRITE_VALUE` are
                // distinct operations with nominally distinct union variants, but
                // in reality they are structurally different, so we take a little
                // shortcut here
                cuda_types::CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WAIT_VALUE_32
                | cuda_types::CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WRITE_VALUE_32 => {
                    write_wait_value(&self.waitValue, writer, false)
                }
                cuda_types::CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WAIT_VALUE_64
                | cuda_types::CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WRITE_VALUE_64 => {
                    write_wait_value(&self.waitValue, writer, true)
                }
                cuda_types::CUstreamBatchMemOpType::CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES => {
                    CudaDisplay::write(&self.flushRemoteWrites, "", 0, writer)
                }
                _ => {
                    writer.write_all(b"{ operation: ")?;
                    CudaDisplay::write(&self.operation, "", 0, writer)?;
                    writer.write_all(b", ... }")
                }
            }
        }
    }
}

pub fn write_wait_value(
    this: &cuda_types::CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st,
    writer: &mut (impl std::io::Write + ?Sized),
    is_64_bit: bool,
) -> std::io::Result<()> {
    writer.write_all(b"{ operation: ")?;
    CudaDisplay::write(&this.operation, "", 0, writer)?;
    writer.write_all(b", address: ")?;
    CudaDisplay::write(&this.address, "", 0, writer)?;
    write_wait_value_32_or_64(&this.__bindgen_anon_1, writer, is_64_bit)?;
    writer.write_all(b", flags: ")?;
    CudaDisplay::write(&this.flags, "", 0, writer)?;
    writer.write_all(b", alias: ")?;
    CudaDisplay::write(&this.alias, "", 0, writer)?;
    writer.write_all(b" }")
}

pub fn write_wait_value_32_or_64(
    this: &cuda_types::CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st__bindgen_ty_1,
    writer: &mut (impl std::io::Write + ?Sized),
    is_64_bit: bool,
) -> std::io::Result<()> {
    if is_64_bit {
        writer.write_all(b", value64: ")?;
        CudaDisplay::write(unsafe { &this.value64 }, "", 0, writer)
    } else {
        writer.write_all(b", value: ")?;
        CudaDisplay::write(unsafe { &this.value }, "", 0, writer)
    }
}

impl CudaDisplay for cuda_types::CUDA_RESOURCE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ resType: ")?;
        CudaDisplay::write(&self.resType, "", 0, writer)?;
        match self.resType {
            cuda_types::CUresourcetype::CU_RESOURCE_TYPE_ARRAY => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.array }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            cuda_types::CUresourcetype::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.mipmap }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            cuda_types::CUresourcetype::CU_RESOURCE_TYPE_LINEAR => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.linear }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            cuda_types::CUresourcetype::CU_RESOURCE_TYPE_PITCH2D => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.pitch2D }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            _ => {
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b", ... }")
            }
        }
    }
}

impl CudaDisplay for cuda_types::CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ type: ")?;
        CudaDisplay::write(&self.type_, "", 0, writer)?;
        match self.type_ {
            cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.fd }, "", 0,writer)?;
            }
            cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32
            | cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP
            | cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE
            |cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE => {
                write_win32_handle(unsafe { self.handle.win32 }, writer)?;
            }
            cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT
            | cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.win32.handle }, "", 0,writer)?;
            }
            cuda_types::CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.nvSciBufObject }, "", 0,writer)?;
            }
            _ => {
                writer.write_all(b", size: ")?;
                CudaDisplay::write(&self.size, "", 0,writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0,writer)?;
                return writer.write_all(b", ... }")
            }
        }
        writer.write_all(b", size: ")?;
        CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(b", flags: ")?;
        CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

pub fn write_win32_handle(
    win32: cuda_types::CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    if win32.handle != ptr::null_mut() {
        writer.write_all(b", handle: ")?;
        CudaDisplay::write(&win32.handle, "", 0, writer)?;
    }
    if win32.name != ptr::null_mut() {
        let name_ptr = win32.name as *const u16;
        let mut strlen = 0usize;
        while unsafe { *name_ptr.add(strlen) } != 0 {
            strlen += 1;
        }
        let text = String::from_utf16_lossy(unsafe { slice::from_raw_parts(name_ptr, strlen) });
        write!(writer, ", name: \"{}\"", text)?;
    }
    Ok(())
}

impl CudaDisplay for cuda_types::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ type: ")?;
        CudaDisplay::write(&self.type_, "", 0, writer)?;
        match self.type_ {
            cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.fd }, "", 0,writer)?;
            }
            cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32
            | cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE
            | cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE
            | cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX
            | cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT => {
                write_win32_handle(unsafe { mem::transmute(self.handle.win32) }, writer)?;
            }
            cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.win32.handle }, "", 0,writer)?;
            }
            cuda_types::CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.nvSciSyncObj }, "", 0,writer)?;
            }
            _ => {
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0,writer)?;
                return writer.write_all(b", ... }")
            }
        }
        writer.write_all(b", flags: ")?;
        CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay
    for cuda_types::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_2
{
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ fence: ")?;
        CudaDisplay::write(&unsafe { self.fence }, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay
    for cuda_types::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_2
{
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ fence: ")?;
        CudaDisplay::write(&unsafe { self.fence }, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl<T: CudaDisplay> CudaDisplay for *mut T {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if *self == ptr::null_mut() {
            writer.write_all(b"NULL")
        } else {
            if fn_name == "cuLaunchKernel" && index == 9 {
                unsafe { write_launch_kernel_extra(*self as *mut *mut c_void, writer) }
            } else {
                let this: &T = unsafe { &**self };
                this.write(fn_name, index, writer)
            }
        }
    }
}

unsafe fn write_launch_kernel_extra(
    mut extra: *mut *mut c_void,
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"[")?;
    let mut format_as_size = false;
    loop {
        let ptr = *extra;
        match ptr as usize {
            0 => {
                writer.write_all(b"CU_LAUNCH_PARAM_END")?;
                break;
            }
            1 => {
                writer.write_all(b"CU_LAUNCH_PARAM_BUFFER_POINTER")?;
            }
            2 => {
                writer.write_all(b"CU_LAUNCH_PARAM_BUFFER_SIZE")?;
                format_as_size = true;
            }
            _ => {
                if format_as_size {
                    let size = *(ptr as *mut usize);
                    write!(writer, "{}", size)?;
                    format_as_size = false;
                } else {
                    write!(writer, "{:p}", ptr)?;
                }
            }
        }
        writer.write_all(b", ")?;
        extra = extra.offset(1);
    }
    writer.write_all(b"]")?;
    Ok(())
}

impl<T: CudaDisplay> CudaDisplay for *const T {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if *self == ptr::null() {
            writer.write_all(b"NULL")
        } else {
            let this: &T = unsafe { &**self };
            this.write(fn_name, index, writer)
        }
    }
}

impl<T: CudaDisplay, const N: usize> CudaDisplay for [T; N] {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"[")?;
        for i in 0..N {
            CudaDisplay::write(&self[i], "", 0, writer)?;
            if i != N - 1 {
                writer.write_all(b", ")?;
            }
        }
        writer.write_all(b"]")
    }
}

impl<const N: usize> CudaDisplay for [i8; N] {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"[")?;
        for i in 0..N {
            write!(writer, "{}", self[i])?;
            if i != N - 1 {
                writer.write_all(b", ")?;
            }
        }
        writer.write_all(b"]")
    }
}

impl CudaDisplay for cuda_types::CUgraphNodeParams {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl CudaDisplay for cuda_types::CUlaunchAttribute_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

#[allow(non_snake_case)]
pub fn write_cuStreamBatchMemOp(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::CUstream,
    count: ::std::os::raw::c_uint,
    paramArray: *mut cuda_types::CUstreamBatchMemOpParams,
    flags: ::std::os::raw::c_uint,
) -> std::io::Result<()> {
    writer.write_all(b"(stream: ")?;
    CudaDisplay::write(&stream, "cuStreamBatchMemOp", 0, writer)?;
    writer.write_all(b", ")?;
    writer.write_all(b"count: ")?;
    CudaDisplay::write(&count, "cuStreamBatchMemOp", 1, writer)?;
    writer.write_all(b", paramArray: [")?;
    for i in 0..count {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        CudaDisplay::write(
            &unsafe { paramArray.add(i as usize) },
            "cuStreamBatchMemOp",
            2,
            writer,
        )?;
    }
    writer.write_all(b"], flags: ")?;
    CudaDisplay::write(&flags, "cuStreamBatchMemOp", 3, writer)?;
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cuGraphKernelNodeGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::CUgraphNode,
    attr: cuda_types::CUlaunchAttributeID,
    value_out: *mut cuda_types::CUkernelNodeAttrValue,
) -> std::io::Result<()> {
    writer.write_all(b"(hNode: ")?;
    CudaDisplay::write(&hNode, "cuGraphKernelNodeGetAttribute", 0, writer)?;
    writer.write_all(b", attr: ")?;
    CudaDisplay::write(&attr, "cuGraphKernelNodeGetAttribute", 1, writer)?;
    match attr {
        cuda_types::CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).accessPolicyWindow },
                "cuGraphKernelNodeGetAttribute",
                2,
                writer,
            )?;
        }
        cuda_types::CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_COOPERATIVE => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).cooperative },
                "cuGraphKernelNodeGetAttribute",
                2,
                writer,
            )?;
        }
        _ => return writer.write_all(b", ...) "),
    }
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cuGraphKernelNodeSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::CUgraphNode,
    attr: cuda_types::CUlaunchAttributeID,
    value_out: *const cuda_types::CUkernelNodeAttrValue,
) -> std::io::Result<()> {
    write_cuGraphKernelNodeGetAttribute(writer, hNode, attr, value_out as *mut _)
}

#[allow(non_snake_case)]
pub fn write_cuStreamGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::CUstream,
    attr: cuda_types::CUstreamAttrID,
    value_out: *mut cuda_types::CUstreamAttrValue,
) -> std::io::Result<()> {
    writer.write_all(b"(hStream: ")?;
    CudaDisplay::write(&hStream, "cuStreamGetAttribute", 0, writer)?;
    writer.write_all(b", attr: ")?;
    CudaDisplay::write(&attr, "cuStreamGetAttribute", 1, writer)?;
    match attr {
        cuda_types::CUstreamAttrID::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).accessPolicyWindow },
                "cuStreamGetAttribute",
                2,
                writer,
            )?;
        }
        cuda_types::CUstreamAttrID::CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).syncPolicy },
                "cuStreamGetAttribute",
                2,
                writer,
            )?;
        }
        _ => return writer.write_all(b", ...) "),
    }
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cuStreamGetAttribute_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::CUstream,
    attr: cuda_types::CUstreamAttrID,
    value_out: *mut cuda_types::CUstreamAttrValue,
) -> std::io::Result<()> {
    write_cuStreamGetAttribute(writer, hStream, attr, value_out)
}

#[allow(non_snake_case)]
pub fn write_cuStreamSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::CUstream,
    attr: cuda_types::CUstreamAttrID,
    value_out: *const cuda_types::CUstreamAttrValue,
) -> std::io::Result<()> {
    write_cuStreamGetAttribute(writer, hStream, attr, value_out as *mut _)
}

#[allow(non_snake_case)]
pub fn write_cuStreamSetAttribute_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::CUstream,
    attr: cuda_types::CUstreamAttrID,
    value_out: *const cuda_types::CUstreamAttrValue,
) -> std::io::Result<()> {
    write_cuStreamSetAttribute(writer, hStream, attr, value_out)
}

#[allow(non_snake_case)]
pub fn write_cuCtxCreate_v3(
    _writer: &mut (impl std::io::Write + ?Sized),
    _pctx: *mut cuda_types::CUcontext,
    _paramsArray: *mut cuda_types::CUexecAffinityParam,
    _numParams: ::std::os::raw::c_int,
    _flags: ::std::os::raw::c_uint,
    _dev: cuda_types::CUdevice,
) -> std::io::Result<()> {
    todo!()
}

#[allow(non_snake_case)]
pub fn write_cuCtxGetExecAffinity(
    _writer: &mut (impl std::io::Write + ?Sized),
    _pExecAffinity: *mut cuda_types::CUexecAffinityParam,
    _type_: cuda_types::CUexecAffinityType,
) -> std::io::Result<()> {
    todo!()
}

#[allow(non_snake_case)]
pub fn write_cuMemMapArrayAsync(
    _writer: &mut (impl std::io::Write + ?Sized),
    _mapInfoList: *mut cuda_types::CUarrayMapInfo,
    _count: ::std::os::raw::c_uint,
    _hStream: cuda_types::CUstream,
) -> std::io::Result<()> {
    todo!()
}

#[allow(non_snake_case)]
pub fn write_cuMemMapArrayAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    mapInfoList: *mut cuda_types::CUarrayMapInfo,
    count: ::std::os::raw::c_uint,
    hStream: cuda_types::CUstream,
) -> std::io::Result<()> {
    write_cuMemMapArrayAsync(writer, mapInfoList, count, hStream)
}

#[allow(non_snake_case)]
pub fn write_cuLinkCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    numOptions: ::std::os::raw::c_uint,
    options: *mut CUjit_option,
    optionValues: *mut *mut ::std::os::raw::c_void,
    stateOut: *mut CUlinkState,
) -> std::io::Result<()> {
    writer.write_all(b"(numOptions: ")?;
    CudaDisplay::write(&numOptions, "cuLinkCreate_v2", 0, writer)?;
    writer.write_all(b", options: ")?;
    write_array("cuLinkCreate_v2", 1, writer, numOptions, options)?;
    writer.write_all(b", optionValues: ")?;
    write_array("cuLinkCreate_v2", 2, writer, numOptions, optionValues)?;
    writer.write_all(b", stateOut: ")?;
    CudaDisplay::write(&stateOut, "cuLinkCreate_v2", 3, writer)?;
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cuLinkAddData_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    state: CUlinkState,
    type_: CUjitInputType,
    data: *mut ::std::os::raw::c_void,
    size: usize,
    name: *const ::std::os::raw::c_char,
    numOptions: ::std::os::raw::c_uint,
    options: *mut CUjit_option,
    optionValues: *mut *mut ::std::os::raw::c_void,
) -> std::io::Result<()> {
    writer.write_all(b"(state: ")?;
    CudaDisplay::write(&state, "cuLinkAddData_v2", 0, writer)?;
    writer.write_all(b", type: ")?;
    CudaDisplay::write(&type_, "cuLinkAddData_v2", 1, writer)?;
    writer.write_all(b", data: ")?;
    CudaDisplay::write(&data, "cuLinkAddData_v2", 2, writer)?;
    writer.write_all(b", size: ")?;
    CudaDisplay::write(&size, "cuLinkAddData_v2", 3, writer)?;
    writer.write_all(b", name: ")?;
    CudaDisplay::write(&name, "cuLinkAddData_v2", 4, writer)?;
    writer.write_all(b", numOptions: ")?;
    CudaDisplay::write(&numOptions, "cuLinkAddData_v2", 5, writer)?;
    writer.write_all(b", options: ")?;
    write_array("cuLinkAddData_v2", 6, writer, numOptions, options)?;
    writer.write_all(b", optionValues: ")?;
    write_array("cuLinkAddData_v2", 7, writer, numOptions, optionValues)?;
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cuModuleLoadDataEx(
    writer: &mut (impl std::io::Write + ?Sized),
    module: *mut CUmodule,
    image: *const ::std::os::raw::c_void,
    numOptions: ::std::os::raw::c_uint,
    options: *mut CUjit_option,
    optionValues: *mut *mut ::std::os::raw::c_void,
) -> std::io::Result<()> {
    writer.write_all(b"(module: ")?;
    CudaDisplay::write(&module, "cuModuleLoadDataEx", 0, writer)?;
    writer.write_all(b", image: ")?;
    CudaDisplay::write(&image, "cuModuleLoadDataEx", 1, writer)?;
    writer.write_all(b", numOptions: ")?;
    CudaDisplay::write(&numOptions, "cuModuleLoadDataEx", 2, writer)?;
    writer.write_all(b", options: ")?;
    write_array("cuModuleLoadDataEx", 3, writer, numOptions, options)?;
    writer.write_all(b", optionValues: ")?;
    write_array("cuModuleLoadDataEx", 4, writer, numOptions, optionValues)?;
    writer.write_all(b")")
}

fn write_array<T: CudaDisplay>(
    fn_name: &'static str,
    index: usize,
    writer: &mut (impl std::io::Write + ?Sized),
    length: u32,
    values: *mut T,
) -> std::io::Result<()> {
    let length = length as usize;
    if length < 2 {
        CudaDisplay::write(&values, fn_name, index, writer)
    } else {
        writer.write_all(b"[")?;
        for i in 0..length {
            CudaDisplay::write(&unsafe { values.add(i) }, fn_name, index, writer)?;
            if i != length - 1 {
                writer.write_all(b", ")?;
            }
        }
        writer.write_all(b"]")
    }
}

cuda_derive_display_trait!(
    cuda_types,
    CudaDisplay,
    [
        CUarrayMapInfo_st,
        CUDA_RESOURCE_DESC_st,
        CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st,
        CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st,
        CUexecAffinityParam_st,
        CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st,
        CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st,
        CUuuid_st,
        HGPUNV,
        CUgraphNodeParams_st,
        CUlaunchAttribute_st
    ],
    [
        cuCtxCreate_v3,
        cuCtxGetExecAffinity,
        cuGraphKernelNodeGetAttribute,
        cuGraphKernelNodeSetAttribute,
        cuMemMapArrayAsync,
        cuMemMapArrayAsync_ptsz,
        cuStreamBatchMemOp,
        cuStreamGetAttribute,
        cuStreamGetAttribute_ptsz,
        cuStreamSetAttribute,
        cuStreamSetAttribute_ptsz,
        cuLinkCreate_v2,
        cuLinkAddData_v2,
        cuModuleLoadDataEx,
    ]
);

#[cfg(test)]

mod tests {
    use cuda_types::CUuuid;

    use super::CudaDisplay;

    #[test]
    fn guid_formats_correctly() {
        let cuid = CUuuid {
            bytes: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
        };
        let mut writer = String::new();
        CudaDisplay::write(&cuid, "", 0, unsafe { writer.as_mut_vec() }).unwrap();
        assert_eq!(writer, "{04030201-0605-0807-090a-0b0c0d0e0f10}");
    }
}
