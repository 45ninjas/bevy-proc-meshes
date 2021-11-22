use bevy::{prelude::*, render::wireframe::Wireframe};

pub struct HeightMapTerrain;

impl Plugin for HeightMapTerrain {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = create_plane(5.0, 40);

    // let texture_handle = asset_server.load("textures/512-uv-grid.png");
    // Spawn a mesh.
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: asset_server.load("textures/vintage-tile/albedo.png").into(),
                metallic_roughness_texture: asset_server.load("textures/vintage-tile/roughness.png").into(),
                normal_map: asset_server.load("textures/vintage-tile/normal-ogl.png").into(),
                occlusion_texture: asset_server.load("textures/vintage-tile/ao.png").into(),
                ..Default::default()
            }),
            ..Default::default()
        });
        // .insert(Wireframe);
}

fn create_plane(size: f32, segments: u32) -> Mesh {
    let vert_step = size / (segments + 1) as f32;
    let uv_step = 1.0 / (segments + 1) as f32;

    let vertex_grid = segments + 1;

    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();
    let mut tangents = Vec::new();
    let mut indices = Vec::new();

    let mut i = 0;

    for y in 0..vertex_grid {
        for x in 0..vertex_grid {
            positions.push([x as f32 * vert_step, 0.0, y as f32 * vert_step]);
            uvs.push([x as f32 * uv_step, y as f32 * uv_step]);
            normals.push([0.0, 1.0, 0.0]);
            tangents.push([1.0, 0.0, 0.0]);

            if x < segments && y < segments {
                indices.push(i);
                indices.push(i + vertex_grid);
                indices.push(i + 1);

                indices.push(i + vertex_grid + 1);
                indices.push(i + 1);
                indices.push(i + vertex_grid);
            }
            i = i + 1;
        }
    }

    let indices = bevy::render::mesh::Indices::U32(indices);

    // Create the actual mesh.
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_TANGENT, tangents);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    return mesh;
}
