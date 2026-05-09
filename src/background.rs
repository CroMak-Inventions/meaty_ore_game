use bevy::{
    light::NotShadowCaster,
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

/// This example uses a shader source file from the assets subdirectory
/// TODO: we will fix this later, putting it in our asset loader.
const SHADER_ASSET_PATH: &str = "shaders/starfield.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
}    

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }    
}    

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup_background);    
    }
}    

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {

    let mut mesh = Sphere::new(100.0).mesh().ico(6).unwrap();
    flip_mesh_normals(&mut mesh);

    commands.spawn((
        Name::new("SkyBox"),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(custom_materials.add(CustomMaterial {
        })),
        NotShadowCaster,
    ));

}

pub fn flip_mesh_normals(mesh: &mut Mesh) {
    if let Some(normals) = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL) {
        if let VertexAttributeValues::Float32x3(values) = normals {
            for n in values.iter_mut() {
                n[0] = -n[0];
                n[1] = -n[1];
                n[2] = -n[2];
            }
        }
    }

    if let Some(indices) = mesh.indices_mut() {
        match indices {
            Indices::U16(vec) => {
                for i in vec.chunks_exact_mut(3) {
                    i.swap(1, 2);
                }
            }
            Indices::U32(vec) => {
                for i in vec.chunks_exact_mut(3) {
                    i.swap(1, 2);
                }
            }
        }
    }
}
