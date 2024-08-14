#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::prepass_utils
#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(
    @builtin(sample_index) sample_index: u32,
    in: VertexOutput
) -> @location(0) vec4f {
    let strength = 1.;
    let frag_coord = in.position;

    let depth = prepass_utils::prepass_depth(frag_coord, sample_index);

    // From Depth Texture explained - Godot https://www.youtube.com/watch?v=wyGWuGQO63Y
    let matrix = view.clip_from_view;
    let remapped_depth = matrix[3][2] / (depth + matrix[2][2]);
    let remapped_z = matrix[3][2] / (frag_coord.z + matrix[2][2]);

    var difference = remapped_depth - remapped_z;
    difference = pow(difference, 0.2);
    difference = smoothstep(strength, 0.0, difference);

    return vec4f(vec3f(1.), difference);
}
