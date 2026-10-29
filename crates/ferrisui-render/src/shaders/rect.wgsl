

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) rect_pos: vec2<f32>,
    @location(1) rect_size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) border_width: f32,
    @location(5) corner_radius: f32,
};



struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) border_width: f32,
    @location(5) corner_radius: f32,

};

struct WindowUniform {
    screen_size: vec2<f32>
};


@group(0) @binding(0)
var<uniform> uniforms: WindowUniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput{
    var out: VertexOutput;

    // Standard 6-vertex quad layout (two triangles)
    var quad_offsets = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );

     let offset = quad_offsets[input.vertex_index];
    let pixel_pos = input.rect_pos + offset * input.rect_size;
    // Convert pixel coordinates (0..width, 0..height) to NDC (-1..1, 1..-1)
    let ndc_x = (pixel_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pixel_pos.y / uniforms.screen_size.y) * 2.0;
    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.local_pos = offset * input.rect_size;
    out.size = input.rect_size;
    out.color = input.color;
    out.border_color = input.border_color;
    out.border_width = input.border_width;
    out.corner_radius = input.corner_radius;
    return out;

}

// 2D Box Signed Distance Field with rounded corners
fn rounded_box_sdf(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r, r);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half_size = in.size * 0.5;
    let p = in.local_pos - half_size;
    let r = min(in.corner_radius, min(half_size.x, half_size.y));
    // Outer edge distance
    let dist = rounded_box_sdf(p, half_size, r);
    let edge_softness = fwidth(dist);
    // Alpha mask for outer rounded box
    let outer_alpha = 1.0 - smoothstep(-edge_softness, 0.0, dist);
    if (in.border_width > 0.0) {
        // Inner edge distance for border
        let inner_dist = rounded_box_sdf(p, half_size - vec2<f32>(in.border_width), max(r - in.border_width, 0.0));
        let inner_alpha = 1.0 - smoothstep(-edge_softness, 0.0, inner_dist);
        // Blend inner fill color and border color
        let fill = in.color * inner_alpha;
        let border = in.border_color * (outer_alpha - inner_alpha);
        return (fill + border);
    }
    return in.color * outer_alpha;
}