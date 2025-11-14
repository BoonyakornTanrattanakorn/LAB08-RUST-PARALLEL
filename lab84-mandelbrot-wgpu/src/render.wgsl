@group(0) @binding(0) var my_sampler: sampler;
@group(0) @binding(1) var my_texture: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) uv: vec2f,
};

var<private> POSITIONS: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(-1.0, -1.0),
    vec2f( 1.0, -1.0),
    vec2f( 1.0,  1.0),

    vec2f(-1.0, -1.0),
    vec2f( 1.0,  1.0),
    vec2f(-1.0,  1.0)
);

var<private> UVS: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 1.0),
    vec2f(1.0, 1.0),
    vec2f(1.0, 0.0),

    vec2f(0.0, 1.0),
    vec2f(1.0, 0.0),
    vec2f(0.0, 0.0)
);


@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    // Create a VertexOutput and fill in the clip_position and uv
    var out: VertexOutput;
    
    // Set the clip position and UV coordinates
    out.clip_position = vec4f(POSITIONS[in_vertex_index][0], POSITIONS[in_vertex_index][1], 0.0, 1.0);
    out.uv = UVS[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    // Sample the texture at the UV coordinates
    // Hint: Use textureSample(texture, sampler, uv_coordinates)

    // Sample and return the texture color
    return textureSample(my_texture, my_sampler, in.uv); // Replace with textureSample
}