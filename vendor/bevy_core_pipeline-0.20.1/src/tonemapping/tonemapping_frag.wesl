import bevy_render::{
    view::View,
};
import package::{
    fullscreen_vertex_shader::FullscreenVertexOutput,
    tonemapping::tone_mapping,
};

@group(0) @binding(0) var<uniform> view: View;

@group(0) @binding(1) var hdr_texture: texture_2d<f32>;
@group(0) @binding(2) var hdr_sampler: sampler;
@group(0) @binding(3) var dt_lut_texture: texture_3d<f32>;
@group(0) @binding(4) var dt_lut_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let hdr_color = textureSample(hdr_texture, hdr_sampler, in.uv);

    return tone_mapping(hdr_color, view.color_grading);
}
