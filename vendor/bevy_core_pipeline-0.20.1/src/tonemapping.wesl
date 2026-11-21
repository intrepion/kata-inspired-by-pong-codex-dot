// Pong's bounded SDR profile. Preserve the default Linear color path only.
import bevy_render::{view::ColorGrading, maths::powsafe};

fn tone_mapping(input_color: vec4<f32>, grading: ColorGrading) -> vec4<f32> {
    let color = max(input_color.rgb, vec3<f32>(0.0)) *
        powsafe(vec3<f32>(2.0), grading.exposure);
    let gray = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    return vec4<f32>(mix(vec3<f32>(gray), color, vec3<f32>(grading.post_saturation)), input_color.a);
}

// Retain the interface for stock shader modules that import it.
fn approximate_inverse_tone_mapping(input_color: vec4<f32>, grading: ColorGrading) -> vec4<f32> {
    let mapped = tone_mapping(input_color, grading);
    let approximate_ratio = length(input_color.rgb) / length(mapped.rgb);
    return vec4<f32>(input_color.rgb * approximate_ratio, input_color.a);
}
