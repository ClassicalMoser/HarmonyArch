// Simple 3D shader with basic lighting
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct Uniforms {
    view_projection: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = model.position;
    out.normal = model.normal;
    out.clip_position = uniforms.view_projection * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_direction = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let normal = normalize(in.normal);
    
    // Diffuse lighting
    let diffuse = max(dot(normal, light_direction), 0.0);
    
    // Ambient lighting
    let ambient = 0.3;
    
    // Final color (white with lighting)
    let color = vec3<f32>(0.9, 0.9, 0.9) * (ambient + diffuse);
    
    return vec4<f32>(color, 1.0);
} 