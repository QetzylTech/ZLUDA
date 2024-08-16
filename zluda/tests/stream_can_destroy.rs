use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(can_destroy_stream);

unsafe fn can_destroy_stream<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut stream = ptr::null_mut();
    assert_eq!(cuda.cuStreamCreate(&mut stream, 0), CUresult::CUDA_SUCCESS);
    assert_eq!(cuda.cuStreamDestroy_v2(stream), CUresult::CUDA_SUCCESS);
    // Cleanup
    assert_eq!(cuda.cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
}
