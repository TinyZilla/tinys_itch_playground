#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::prepass_utils
#import bevy_pbr::utils
#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(
    @builtin(sample_index) sample_index: u32,
    @builtin(front_facing) is_front: bool,
    in: VertexOutput
) -> @location(0) vec4f {
    let frag_coord = in.position;

    let viewport_uv = utils::coords_to_viewport_uv(frag_coord.xy, view.viewport);
    let depth = prepass_utils::prepass_depth(frag_coord, sample_index);

    let distance = 0.5; // Fade Distance in Clip Space....
    let ramp_factor = 5.;

    let intersection_mask = intersection_mask(frag_coord.z, depth, view.clip_from_view, distance, ramp_factor);

    let is_front_f32 = f32(is_front);

    // Mock Fresnel Implimentation -- https://www.youtube.com/watch?v=a66SysxGebo
    let V = normalize(view.world_position.xyz - in.world_position.xyz);
    let NdotV = max(dot(in.world_normal , V), 0.00001);
    // let NdotV = max(dot(normalize(in.world_normal) , V), 0.00001);

    let mock_fresnel = clamp(1.0 - NdotV, 0.0, 1.0) * is_front_f32;

    let ramped_fresnel = pow(mock_fresnel, 1.);

    let composit = max(ramped_fresnel, intersection_mask * (is_front_f32 * 0.8 + 0.2));
    let color1 = vec3f(0.0, 1.0, 0.0);
    let color2 = vec3f(1.0, 0.0, 1.0);
    // var out_color = mix(color_1, color_2);
    // out_color = frag_coord.xxx * frag_coord.w;
    return vec4f(mix(color1, color2, composit), 1.0);
}

// From Depth Texture explained - Godot https://www.youtube.com/watch?v=wyGWuGQO63Y
// distance_clip -- distance in clip space (1.0 is 1 unit)
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
