#[macro_export]
macro_rules! include_wgsl_shader {
    // Initial entry for the macro with shader file and module name
    ($shader_path:expr, $mod_name:ident $(, $entry_point:ident as $descriptor:ident)*) => {
        paste::paste! {
            pub mod $mod_name {
                #[include_wgsl_oil::include_wgsl_oil($shader_path)]
                pub mod naga {}

                // Process each entry point and descriptor pair
                $( $crate::include_wgsl_shader!(@inner $entry_point as $descriptor); )*
            }
        }
    };
    // Inner macro for handling each shader entry point and descriptor pair
    (@inner $entry_point:ident as $descriptor:ident) => {
        paste::paste! {
            pub const $descriptor: wgpu::ShaderModuleDescriptor =
                wgpu::ShaderModuleDescriptor {
                    label: Some(concat!(stringify!($mod_name), "::", stringify!($entry_point))),
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                        naga::entry_points::$entry_point::EXCLUSIVE_SOURCE,
                    )),
                };
        }
    };
}

#[macro_export]
macro_rules! include_wgsl_shader_vertex_fragment {
    ($shader_path:expr, $mod_name:ident) => {
        include_wgsl_shader!(
            $shader_path,
            $mod_name,
            vs_main as SHADER_DESCRIPTOR_VERTEX,
            fs_main as SHADER_DESCRIPTOR_FRAGMENT
        );
    };
}
