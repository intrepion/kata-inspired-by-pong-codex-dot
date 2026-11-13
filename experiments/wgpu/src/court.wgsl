struct State { positions: vec4<f32>, viewport_scores: vec4<f32>, status: vec4<f32> }
@group(0) @binding(0) var<uniform> state: State;
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    var p = array<vec2<f32>, 3>(vec2(-1., -1.), vec2(3., -1.), vec2(-1., 3.));
    return vec4(p[i], 0., 1.);
}
fn glyph(ch: u32) -> array<u32, 7> {
    switch ch {
        case 65u: { return array<u32,7>(14,17,17,31,17,17,17); }
        case 67u: { return array<u32,7>(14,17,16,16,16,17,14); }
        case 68u: { return array<u32,7>(30,17,17,17,17,17,30); }
        case 69u: { return array<u32,7>(31,16,16,30,16,16,31); }
        case 73u: { return array<u32,7>(14,4,4,4,4,4,14); }
        case 78u: { return array<u32,7>(17,25,21,19,17,17,17); }
        case 79u: { return array<u32,7>(14,17,17,17,17,17,14); }
        case 80u: { return array<u32,7>(30,17,17,30,16,16,16); }
        case 82u: { return array<u32,7>(30,17,17,30,20,18,17); }
        case 83u: { return array<u32,7>(15,16,16,14,1,1,30); }
        case 85u: { return array<u32,7>(17,17,17,17,17,17,14); }
        case 86u: { return array<u32,7>(17,17,17,17,17,10,4); }
        case 87u: { return array<u32,7>(17,17,17,21,21,21,10); }
        case 89u: { return array<u32,7>(17,17,10,4,4,4,4); }
        default: { return array<u32,7>(0,0,0,0,0,0,0); }
    }
}
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    let size = state.viewport_scores.xy;
    let scale = min(size.x / 900., size.y / 500.);
    let p = (pixel.xy - (size - vec2(900.,500.) * scale) * 0.5) / scale;
    var c = vec3(0.025,0.04,0.09);
    if any(p < vec2(0.)) || any(p > vec2(900.,500.)) { return vec4(0.,0.,0.,1.); }
    if abs(p.x-450.) < 1. && (u32(p.y)%25u) < 12u { c = vec3(0.18,0.25,0.35); }
    if (abs(p.x-38.) < 6. && abs(p.y-state.positions.x) < 45.) || (abs(p.x-862.) < 6. && abs(p.y-state.positions.y) < 45.) { c = vec3(0.35,0.95,0.85); }
    if distance(p,state.positions.zw) < 8. { c = vec3(1.); }
    for (var side=0u; side<2u; side++) {
        let score = select(state.viewport_scores.z,state.viewport_scores.w,side==1u);
        for (var j=0u; j<u32(score); j++) {
            let x = 450. + (f32(side)*2.-1.) * (30. + f32(j)*16.);
            if abs(p.x-x)<4. && abs(p.y-24.)<8. { c=vec3(1.); }
        }
    }
    // Identify the two score groups in the court, including when the title is hidden.
    for (var side=0u; side<2u; side++) {
        let left = select(354., 510., side==1u);
        if p.x >= left && p.x < left+36. && p.y >= 38. && p.y < 52. {
            let x = u32((p.x-left)/2.);
            let row = u32((p.y-38.)/2.);
            let col = x % 6u;
            var label = array<u32,3>(89,79,85);
            if side == 1u { label = array<u32,3>(67,80,85); }
            if col < 5u {
                let bits = glyph(label[x/6u]);
                if (bits[row] & (1u << (4u-col))) != 0u { c = vec3(0.7,0.8,0.9); }
            }
        }
    }
    let code = u32(state.status.x);
    var chars = array<u32,8>(0,0,0,0,0,0,0,0);
    var count = 0u;
    switch code {
        case 1u: { chars = array<u32,8>(82,69,65,68,89,0,0,0); count = 5u; }
        case 2u: { chars = array<u32,8>(80,65,85,83,69,68,0,0); count = 6u; }
        case 3u: { chars = array<u32,8>(89,79,85,0,87,73,78,0); count = 7u; }
        case 4u: { chars = array<u32,8>(67,80,85,0,87,73,78,83); count = 8u; }
        case 5u: { chars = array<u32,8>(79,86,69,82,0,0,0,0); count = 4u; }
        default: {}
    }
    if count > 0u {
        let left = (900. - f32(count)*24.) * 0.5;
        if p.x >= left-8. && p.x < left+f32(count)*24.+8. && p.y >= 152. && p.y < 196. {
            c = vec3(0.025,0.04,0.09);
        }
        if p.x >= left && p.x < left+f32(count)*24. && p.y >= 160. && p.y < 188. {
            let x = u32((p.x-left)/4.);
            let row = u32((p.y-160.)/4.);
            let col = x % 6u;
            if col < 5u {
                let bits = glyph(chars[x/6u]);
                if (bits[row] & (1u << (4u-col))) != 0u { c = vec3(1.,0.7,0.3); }
            }
        }
    }
    return vec4(c,1.);
}
