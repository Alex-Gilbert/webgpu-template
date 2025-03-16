use std::f32::consts::PI;
use wgpu::Device;

use crate::{
    ecs::components::mesh_filter::{BasicMeshFilter, MeshFilter},
    gpu_resources::types::basic_vertex::BasicVertex,
};

/// Creates a plane mesh on the XZ plane with a specified size.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `width` - The width of the plane along the X axis
/// * `depth` - The depth of the plane along the Z axis
/// * `width_segments` - Number of segments along the width
/// * `depth_segments` - Number of segments along the depth
pub fn create_plane(
    device: &Device,
    width: f32,
    depth: f32,
    width_segments: u32,
    depth_segments: u32,
) -> BasicMeshFilter {
    let width_half = width / 2.0;
    let depth_half = depth / 2.0;

    let grid_x = width_segments;
    let grid_z = depth_segments;

    let segment_width = width / (grid_x as f32);
    let segment_depth = depth / (grid_z as f32);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for z in 0..=grid_z {
        let z_pos = z as f32 * segment_depth - depth_half;

        for x in 0..=grid_x {
            let x_pos = x as f32 * segment_width - width_half;

            // Create vertex at this grid position
            vertices.push(BasicVertex {
                position: [x_pos, 0.0, z_pos].into(),
                tex_coords: [x as f32 / grid_x as f32, z as f32 / grid_z as f32].into(),
            });
        }
    }

    // Generate indices for triangles
    for z in 0..grid_z {
        for x in 0..grid_x {
            let a = (z * (grid_x + 1)) + x;
            let b = a + 1;
            let c = a + (grid_x + 1);
            let d = c + 1;

            // Generate two triangles for each grid cell
            indices.push(a);
            indices.push(c);
            indices.push(b);

            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    BasicMeshFilter {
        filter: MeshFilter::new(device, &vertices, &indices),
    }
}

/// Creates a cube mesh with a specified size.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `size` - The size of the cube in all dimensions
/// * `segments` - Number of segments along each edge
pub fn create_cube(device: &Device, size: f32, segments: u32) -> BasicMeshFilter {
    let half_size = size / 2.0;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Function to create a single face of the cube
    let create_face = |vertices: &mut Vec<BasicVertex>,
                       indices: &mut Vec<u32>,
                       normal: [f32; 3],
                       segments: u32,
                       base_index: u32| {
        // Determine tangent and bitangent based on normal
        let tangent = if normal[0].abs() > 0.0 {
            [0.0, 0.0, 1.0]
        } else {
            [1.0, 0.0, 0.0]
        };

        let bitangent = [
            normal[1] * tangent[2] - normal[2] * tangent[1],
            normal[2] * tangent[0] - normal[0] * tangent[2],
            normal[0] * tangent[1] - normal[1] * tangent[0],
        ];

        // Generate vertices
        for j in 0..=segments {
            let v = j as f32 / segments as f32;

            for i in 0..=segments {
                let u = i as f32 / segments as f32;

                // Position on the face
                let x = normal[0] * half_size
                    + tangent[0] * (u - 0.5) * size
                    + bitangent[0] * (v - 0.5) * size;
                let y = normal[1] * half_size
                    + tangent[1] * (u - 0.5) * size
                    + bitangent[1] * (v - 0.5) * size;
                let z = normal[2] * half_size
                    + tangent[2] * (u - 0.5) * size
                    + bitangent[2] * (v - 0.5) * size;

                // Add vertex
                vertices.push(BasicVertex {
                    position: [x, y, z].into(),
                    tex_coords: [u, v].into(),
                });
            }
        }

        // Generate indices
        let vertex_per_row = segments + 1;
        for j in 0..segments {
            for i in 0..segments {
                let a = base_index + i + j * vertex_per_row;
                let b = base_index + i + (j + 1) * vertex_per_row;
                let c = base_index + (i + 1) + (j + 1) * vertex_per_row;
                let d = base_index + (i + 1) + j * vertex_per_row;

                // Add two triangles
                indices.push(a);
                indices.push(b);
                indices.push(d);

                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }

        // Return the number of vertices added
        ((segments + 1) * (segments + 1)) as u32
    };

    // Create 6 faces of the cube
    let mut base_index = 0;

    // Right face (+X)
    base_index += create_face(
        &mut vertices,
        &mut indices,
        [1.0, 0.0, 0.0],
        segments,
        base_index,
    );
    // Left face (-X)
    base_index += create_face(
        &mut vertices,
        &mut indices,
        [-1.0, 0.0, 0.0],
        segments,
        base_index,
    );
    // Top face (+Y)
    base_index += create_face(
        &mut vertices,
        &mut indices,
        [0.0, 1.0, 0.0],
        segments,
        base_index,
    );
    // Bottom face (-Y)
    base_index += create_face(
        &mut vertices,
        &mut indices,
        [0.0, -1.0, 0.0],
        segments,
        base_index,
    );
    // Front face (+Z)
    base_index += create_face(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 1.0],
        segments,
        base_index,
    );
    // Back face (-Z)
    create_face(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, -1.0],
        segments,
        base_index,
    );

    BasicMeshFilter {
        filter: MeshFilter::new(device, &vertices, &indices),
    }
}

/// Creates a sphere mesh with a specified radius.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `radius` - The radius of the sphere
/// * `width_segments` - Number of segments around the equator
/// * `height_segments` - Number of segments from pole to pole
pub fn create_sphere(
    device: &Device,
    radius: f32,
    width_segments: u32,
    height_segments: u32,
) -> BasicMeshFilter {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for y in 0..=height_segments {
        let v = y as f32 / height_segments as f32;
        let phi = v * PI;

        for x in 0..=width_segments {
            let u = x as f32 / width_segments as f32;
            let theta = u * 2.0 * PI;

            // Calculate position on sphere
            let x_pos = -radius * phi.sin() * theta.cos();
            let y_pos = radius * phi.cos();
            let z_pos = radius * phi.sin() * theta.sin();

            vertices.push(BasicVertex {
                position: [x_pos, y_pos, z_pos].into(),
                tex_coords: [u, v].into(),
            });
        }
    }

    // Generate indices
    for y in 0..height_segments {
        for x in 0..width_segments {
            let a = y * (width_segments + 1) + x;
            let b = a + 1;
            let c = a + (width_segments + 1);
            let d = c + 1;

            // For the first row, we only need one triangle per sector
            if y != 0 {
                indices.push(a);
                indices.push(c);
                indices.push(b);
            }

            // For the last row, we only need one triangle per sector
            if y != height_segments - 1 {
                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }
    }

    BasicMeshFilter {
        filter: MeshFilter::new(device, &vertices, &indices),
    }
}

/// Creates a capsule mesh with specified radius and height.
///
/// A capsule is a cylinder with hemispherical caps at both ends.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `radius` - The radius of the capsule
/// * `height` - The height of the cylindrical section (total height = height + 2*radius)
/// * `radial_segments` - Number of segments around the circumference
/// * `height_segments` - Number of segments along the height of the cylindrical section
/// * `cap_segments` - Number of segments for each hemispherical cap
pub fn create_capsule(
    device: &Device,
    radius: f32,
    height: f32,
    radial_segments: u32,
    height_segments: u32,
    cap_segments: u32,
) -> BasicMeshFilter {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let half_height = height / 2.0;

    // Generate top hemisphere vertices
    for y in 0..=cap_segments {
        let v = y as f32 / cap_segments as f32;
        let phi = v * PI / 2.0; // Only going from 0 to π/2 for the top half
        let y_pos = radius * phi.cos() + half_height;
        let radius_at_phi = radius * phi.sin();

        for x in 0..=radial_segments {
            let u = x as f32 / radial_segments as f32;
            let theta = u * 2.0 * PI;

            let x_pos = radius_at_phi * theta.cos();
            let z_pos = radius_at_phi * theta.sin();

            vertices.push(BasicVertex {
                position: [x_pos, y_pos, z_pos].into(),
                tex_coords: [u, v / 2.0].into(), // Map to top quarter of texture
            });
        }
    }

    // Generate cylinder vertices
    for y in 0..=height_segments {
        let v = y as f32 / height_segments as f32;
        let y_pos = (1.0 - v) * half_height - v * half_height;

        for x in 0..=radial_segments {
            let u = x as f32 / radial_segments as f32;
            let theta = u * 2.0 * PI;

            let x_pos = radius * theta.cos();
            let z_pos = radius * theta.sin();

            vertices.push(BasicVertex {
                position: [x_pos, y_pos, z_pos].into(),
                tex_coords: [u, 0.25 + v * 0.5].into(), // Map to middle half of texture
            });
        }
    }

    // Generate bottom hemisphere vertices
    for y in 0..=cap_segments {
        let v = y as f32 / cap_segments as f32;
        let phi = v * PI / 2.0 + PI / 2.0; // Going from π/2 to π for the bottom half
        let y_pos = radius * phi.cos() - half_height;
        let radius_at_phi = radius * phi.sin();

        for x in 0..=radial_segments {
            let u = x as f32 / radial_segments as f32;
            let theta = u * 2.0 * PI;

            let x_pos = radius_at_phi * theta.cos();
            let z_pos = radius_at_phi * theta.sin();

            vertices.push(BasicVertex {
                position: [x_pos, y_pos, z_pos].into(),
                tex_coords: [u, 0.75 + v / 2.0].into(), // Map to bottom quarter of texture
            });
        }
    }

    // Helper function to generate indices for a grid
    let generate_grid_indices =
        |start_index: u32, width_segments: u32, height_segments: u32, indices: &mut Vec<u32>| {
            for y in 0..height_segments {
                for x in 0..width_segments {
                    let a = start_index + y * (width_segments + 1) + x;
                    let b = a + 1;
                    let c = a + (width_segments + 1);
                    let d = c + 1;

                    indices.push(a);
                    indices.push(c);
                    indices.push(b);

                    indices.push(b);
                    indices.push(c);
                    indices.push(d);
                }
            }
        };

    // Generate indices for top hemisphere
    generate_grid_indices(0, radial_segments, cap_segments, &mut indices);

    // Generate indices for cylinder
    let cylinder_start = (cap_segments + 1) * (radial_segments + 1);
    generate_grid_indices(
        cylinder_start,
        radial_segments,
        height_segments,
        &mut indices,
    );

    // Generate indices for bottom hemisphere
    let bottom_start = cylinder_start + (height_segments + 1) * (radial_segments + 1);
    generate_grid_indices(bottom_start, radial_segments, cap_segments, &mut indices);

    BasicMeshFilter {
        filter: MeshFilter::new(device, &vertices, &indices),
    }
}

/// Creates a cylinder mesh with a specified radius and height.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `radius_top` - The radius at the top of the cylinder
/// * `radius_bottom` - The radius at the bottom of the cylinder
/// * `height` - The height of the cylinder
/// * `radial_segments` - Number of segments around the circumference
/// * `height_segments` - Number of segments along the height
/// * `open_ended` - Whether to include the top and bottom caps
pub fn create_cylinder(
    device: &Device,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    radial_segments: u32,
    height_segments: u32,
    open_ended: bool,
) -> BasicMeshFilter {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let half_height = height / 2.0;

    // Generate vertices for the sides of the cylinder
    for y in 0..=height_segments {
        let v = y as f32 / height_segments as f32;
        let y_pos = height * v - half_height;

        // Linearly interpolate between the top and bottom radii
        let radius = radius_bottom + (radius_top - radius_bottom) * v;

        for x in 0..=radial_segments {
            let u = x as f32 / radial_segments as f32;
            let theta = u * 2.0 * PI;

            let x_pos = radius * theta.cos();
            let z_pos = radius * theta.sin();

            vertices.push(BasicVertex {
                position: [x_pos, y_pos, z_pos].into(),
                tex_coords: [u, v].into(),
            });
        }
    }

    // Generate indices for the sides
    for y in 0..height_segments {
        for x in 0..radial_segments {
            let a = y * (radial_segments + 1) + x;
            let b = a + 1;
            let c = a + (radial_segments + 1);
            let d = c + 1;

            indices.push(a);
            indices.push(c);
            indices.push(b);

            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    // If the cylinder is not open-ended, add top and bottom caps
    if !open_ended {
        let mut add_cap = |top: bool| {
            let radius = if top { radius_top } else { radius_bottom };
            let y_pos = if top { half_height } else { -half_height };
            let center_index = vertices.len() as u32;

            // Add center vertex
            vertices.push(BasicVertex {
                position: [0.0, y_pos, 0.0].into(),
                tex_coords: [0.5, 0.5].into(),
            });

            // Add perimeter vertices
            for x in 0..=radial_segments {
                let u = x as f32 / radial_segments as f32;
                let theta = u * 2.0 * PI;

                let x_pos = radius * theta.cos();
                let z_pos = radius * theta.sin();

                vertices.push(BasicVertex {
                    position: [x_pos, y_pos, z_pos].into(),
                    tex_coords: [(theta.cos() + 1.0) / 2.0, (theta.sin() + 1.0) / 2.0].into(),
                });
            }

            // Add triangles
            for x in 0..radial_segments {
                let a = center_index;
                let b = center_index + 1 + x;
                let c = center_index + 1 + ((x + 1) % radial_segments);

                if top {
                    indices.push(a);
                    indices.push(b);
                    indices.push(c);
                } else {
                    indices.push(a);
                    indices.push(c);
                    indices.push(b);
                }
            }
        };

        // Add top and bottom caps
        add_cap(true);
        add_cap(false);
    }

    BasicMeshFilter {
        filter: MeshFilter::new(device, &vertices, &indices),
    }
}

/// Creates a torus mesh with specified radii.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `radius` - The radius from the center of the torus to the center of the tube
/// * `tube_radius` - The radius of the tube
/// * `radial_segments` - Number of segments around the circumference of the torus
/// * `tubular_segments` - Number of segments around the tube
pub fn create_torus(
    device: &Device,
    radius: f32,
    tube_radius: f32,
    radial_segments: u32,
    tubular_segments: u32,
) -> BasicMeshFilter {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for j in 0..=radial_segments {
        for i in 0..=tubular_segments {
            let u = i as f32 / tubular_segments as f32 * 2.0 * PI;
            let v = j as f32 / radial_segments as f32 * 2.0 * PI;

            // Calculate position
            let x = (radius + tube_radius * v.cos()) * u.cos();
            let y = tube_radius * v.sin();
            let z = (radius + tube_radius * v.cos()) * u.sin();

            vertices.push(BasicVertex {
                position: [x, y, z].into(),
                tex_coords: [
                    i as f32 / tubular_segments as f32,
                    j as f32 / radial_segments as f32,
                ]
                .into(),
            });
        }
    }

    // Generate indices
    for j in 0..radial_segments {
        for i in 0..tubular_segments {
            let a = (tubular_segments + 1) * j + i;
            let b = a + 1;
            let c = a + (tubular_segments + 1);
            let d = c + 1;

            indices.push(a);
            indices.push(c);
            indices.push(b);

            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    BasicMeshFilter {
        filter: MeshFilter::new(device, &vertices, &indices),
    }
}

/// Creates a cone mesh with a specified radius and height.
///
/// # Arguments
/// * `device` - The WGPU device to create buffers on
/// * `radius` - The radius at the base of the cone
/// * `height` - The height of the cone
/// * `radial_segments` - Number of segments around the circumference
/// * `height_segments` - Number of segments along the height
/// * `open_ended` - Whether to include the base cap
pub fn create_cone(
    device: &Device,
    radius: f32,
    height: f32,
    radial_segments: u32,
    height_segments: u32,
    open_ended: bool,
) -> BasicMeshFilter {
    // A cone is just a cylinder with radius_top = 0
    create_cylinder(
        device,
        0.0,
        radius,
        height,
        radial_segments,
        height_segments,
        open_ended,
    )
}
