struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Dim {
    width: f32,
    height: f32,
    rotation: mat4x4<f32>,
}

struct GgezDrawUniforms {
    color: vec4<f32>,
    src_rect: vec4<f32>,
    transform: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: GgezDrawUniforms;

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

@group(3) @binding(0)
var<uniform> dim: Dim;

const pi: f32 = 3.1415926535897932384626433832795;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let inner_x: f32 = fract(in.uv.x * dim.width);
    let inner_y: f32 = fract(in.uv.y * dim.height);

    let sin_blob: f32 = sin(inner_x * pi) * sin(inner_y * pi);
    let blob_mask: f32 = smoothstep(0.7, 0.75, sin_blob);

    let tex: vec3<f32> = textureSample(t, s, in.uv).rgb;
    let pix: vec3<f32> = (tex * blob_mask) + (tex * (1.0 - blob_mask)) * 0.05;

    // return textureSample(t, s, in.uv) * in.color * dim.rate;
    return vec4(pix, 1.0);
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    // out.uv = uv * uniforms.src_rect.zw + uniforms.src_rect.xy;
    out.uv = mix(uniforms.src_rect.xy, uniforms.src_rect.zw, uv);
    out.color = color; // which color? uniforms.color or color?
    // out.position = vec4<f32>(position, 0.0, 1.0);
    out.position = uniforms.transform * vec4<f32>(position, 0.0, 1.0);
    



    // out.position = uniforms.transform * my_uniforms.rotation * vec4<f32>(position, 0.0, 1.0);
    // out.uv = mix(uniforms.src_rect.xy, uniforms.src_rect.zw, uv);
    // out.color = uniforms.color * color;
    return out;
}