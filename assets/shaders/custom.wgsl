struct ShaderMaterial {
    time: f32
};

@group(1) @binding(0)
var<uniform> material: ShaderMaterial;

// @group(1) @binding(0)
// var<uniform> material: CustomMaterial;
// @group(1) @binding(1)
// var base_color_texture: texture_2d<f32>;
// @group(1) @binding(2)
// var base_color_sampler: sampler;

@fragment
fn fragment0(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    // return material.color * textureSample(base_color_texture, base_color_sampler, uv);
    return vec4(1., 0.5, 1., 1.);
}

@fragment
fn fragment1(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {    
    return vec4(uv.x, uv.y, 1., 1.);
}