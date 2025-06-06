@export
struct FontVertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) altas_coords: vec2<f32>,
    @location(3) glyph_coords: vec2<f32>,
    @location(4) bounds_coords: vec2<f32>,
}

