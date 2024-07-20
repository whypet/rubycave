@group(0) @binding(0) var<uniform> vp: mat4x4<f32>;
@group(0) @binding(1) var<uniform> atlas: texture_2d<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {}
