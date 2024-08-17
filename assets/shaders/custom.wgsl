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
    let fresnel_power_factor = 4.0;
    let intersection_back_factor = 0.7;

    let intersection_mask = intersection_mask(frag_coord.z, depth, view.clip_from_view, distance, ramp_factor);
    let fresnel = simple_fersnel(view.world_position.xyz, in.world_position.xyz, in.world_normal);

    let is_front_f32 = f32(is_front);
    let ramped_fresnel = pow(fresnel, fresnel_power_factor) * is_front_f32;
    let front_back_scalar = is_front_f32 * (1.0 - intersection_back_factor) + intersection_back_factor;

    let composit = max(ramped_fresnel, intersection_mask * front_back_scalar);
    let color1 = vec3f(1.0, 1.0, 1.0);
    return vec4f(color1, composit * 0.8);
}

// Mock Fresnel Implimentation -- https://www.youtube.com/watch?v=a66SysxGebo
fn simple_fersnel(camera_position: vec3f, mesh_position: vec3f, mesh_normal: vec3f) -> f32 {
    // Get the View Vector
    let V = normalize(camera_position - mesh_position);
    let NdotV = max(dot(mesh_normal , V), 0.00001);
    return clamp(1.0 - NdotV, 0.0, 1.0);
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
