#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::prepass_utils
#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(
    @builtin(sample_index) sample_index: u32,
    in: VertexOutput
) -> @location(0) vec4f {
    let frag_coord = in.position;

    let distance = 0.5; // Fade Distance in Clip Space....
    let ramp_factor = 5.;

    let depth = prepass_utils::prepass_depth(frag_coord, sample_index);

    let intersection_mask = intersection_mask(frag_coord.z, depth, view.clip_from_view, distance, ramp_factor);

    return vec4f(vec3f(1.0), intersection_mask);
}

// From Depth Texture explained - Godot https://www.youtube.com/watch?v=wyGWuGQO63Y
fn intersection_mask(z_ndc: f32, depth_ndc: f32, projection_matrix: mat4x4<f32>, distance_clip: f32, ramp_factor: f32) -> f32 {
    // Remap from Normalized Device Coord to Clip Coord (Only for Z axis)
    let z_clip = projection_matrix[3][2] / (z_ndc + projection_matrix[2][2]);
    let depth_clip = projection_matrix[3][2] / (depth_ndc + projection_matrix[2][2]);

    // Step 1. Calculate the difference between the two.
    // Step 2. Use Smooth Step to remap the value to the desired distance in clip / world space.
    // Step 3. Use pow() with a 1 / ramp_factor to alter the linear curve. (if hard coding -- use sqrt() chained to speed up the calculation.)
    // Step 4. Invert the result so the mask is 1.0 closest to intersection.
    let intersection_mask = 1.0 - pow(smoothstep(0.0, distance_clip, depth_clip - z_clip), 1.0 / ramp_factor);
    return intersection_mask;
}
