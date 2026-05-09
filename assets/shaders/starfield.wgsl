#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}

// Returns a single f32 for a position
fn rand(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(54.90898, 18.233))) * 4337.5453);
}

// Returns two f32 for a position
fn rand2(p: vec2<f32>) -> vec2<f32> {
    let p2 = vec2<f32>(dot(p, vec2<f32>(12.9898, 78.233)), dot(p, vec2<f32>(26.65125, 83.054543)));
    return fract(sin(p2) * 43758.5453);
}

fn stars(position: vec2<f32>, density: f32, size: f32, brightness: f32) -> f32 {
    let n = position * density;
    let f = floor(n);

    var d = 1.0e10;
    for (var i = -1; i <= 1; i = i + 1) {
        for (var j = -1; j <= 1; j = j + 1) {
            var g = f + vec2<f32>(f32(i), f32(j));
            g = n - g - rand2(g % density) + rand(g);
            g = g / (density * size);
            d = min(d, dot(g, g));
        }
    }

    return brightness * (smoothstep(.95, 1., (1. - sqrt(d))));
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let viewport_uv = coords_to_viewport_uv(mesh.position.xy, view.viewport);
    //let move_factor = 1000.0;

    var result = vec3<f32>(0.0, 0.0, 0.0);
    let move_factor = 1000.0;

    //result = result + stars(viewport_uv, 30.0, 0.025, 0.5);

    result = result + stars(viewport_uv, 2.0, 0.025, 2.0);
    result = result + stars(viewport_uv, 6.0, 0.018, 1.0);
    result = result + stars(viewport_uv, 20.0, 0.015, 0.5);

    return vec4<f32>(result, 1.0);

}
