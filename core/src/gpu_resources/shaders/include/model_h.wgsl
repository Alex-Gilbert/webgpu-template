// Model header file containing the core data structure for model transformations
// This defines what model data is available to the shader system

@export struct ModelUniform {
    // The model matrix transforms vertices from model space to world space
    model: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
}
