/// # Safety
/// This trait should only be implemented by the below macros
pub unsafe trait GpuType {}

/// # Safety
/// This trait should only be implemented by the below macros
pub unsafe trait GpuUniformType: GpuType {
    fn as_buffer(&self) -> Vec<u8>;
}

unsafe impl<T> GpuUniformType for T
where
    T: encase::ShaderType + GpuType + encase::internal::WriteInto,
{
    fn as_buffer(&self) -> Vec<u8> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).unwrap();

        buffer.into_inner()
    }
}

#[macro_export]
macro_rules! define_gpu_data_type {
    ($original:path as $alias:ident) => {
        pub use $original as $alias;

        unsafe impl $crate::gpu_resources::types::gpu_type_macros::GpuType for $alias {}
    };
}
