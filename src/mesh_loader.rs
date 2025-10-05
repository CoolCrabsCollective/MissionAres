use crate::hentai_anime::Animation;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::prelude::{Collider, CollisionGroups};

pub struct MeshLoaderPlugin;

/// This type encapsulates what to do with a loaded GLTF asset. In previous jams, we used ugly
/// if-statements inside process_loaded_gltfs to decide how to load things by checking the name with
/// a string, e.g. if a pumpkin, add a point light, etc.
/// I propose we expand on GLTFLoadConfig to provide any extra functionality we need
/// process_loaded_gltfs to do, but while keeping the object-specific logic outside of this class.
pub struct GLTFLoadConfig {
    /// Whether to spawn the loaded GLTF or not during load
    pub spawn: bool,
    /// initializes the entity that was spawned (allows adding bundle, components or do whatever)
    pub entity_initializer: Box<dyn Fn(&mut EntityCommands) + Send + Sync>,
    /// Whether to generate a (static, non rigid body) collider for the loaded GLTF during load
    pub generate_static_collider: bool,
    /// CollisionGroups to use for the generated collider
    pub collision_groups: CollisionGroups,
}

impl Default for GLTFLoadConfig {
    fn default() -> Self {
        Self {
            spawn: true,
            entity_initializer: Box::new(|commands| {}),
            generate_static_collider: false,
            collision_groups: CollisionGroups::default(),
        }
    }
}

pub struct LoadedGLTF {
    pub gltf_handle: Handle<Gltf>,
    pub config: GLTFLoadConfig,
    pub processed: bool,
}

#[derive(Resource)]
pub struct MeshLoader(Vec<LoadedGLTF>);

impl Plugin for MeshLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, process_loaded_gltfs.after(setup));
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(MeshLoader(vec![]));
}

pub fn load_gltf(
    asset_path: String,
    config: GLTFLoadConfig,
    asset_server: &Res<AssetServer>,
    mesh_loader: &mut ResMut<MeshLoader>,
) {
    mesh_loader.0.push(LoadedGLTF {
        gltf_handle: asset_server.load(asset_path),
        processed: false,
        config,
    });
}

#[allow(clippy::too_many_arguments)]
fn process_loaded_gltfs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    nodes: Res<Assets<GltfNode>>,
    mut mesh_loader: ResMut<MeshLoader>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    for loaded_gltf in mesh_loader.0.iter_mut() {
        if loaded_gltf.processed {
            continue;
        }

        let Some(gltf) = gltf_assets.get(&loaded_gltf.gltf_handle) else {
            continue;
        };

        let first_scene_handle = gltf.scenes[0].clone();

        for (name, node_handle) in &gltf.named_nodes {
            if loaded_gltf.config.generate_static_collider {
                info!("Generating collider from level object: {name:?}");
                if let (Some(mesh), Some(material_handle), Some(transform)) = (
                    get_mesh_from_gltf_node(node_handle, &meshes, &gltf_meshes, &nodes),
                    get_material_from_gltf_node(node_handle, &gltf_meshes, &nodes),
                    nodes.get(node_handle).map(|node| node.transform),
                ) {
                    match get_collider_from_mesh(mesh, &transform) {
                        Ok(collider) => {
                            commands
                                .spawn(collider)
                                .insert(loaded_gltf.config.collision_groups);
                        }
                        Err(err) => {
                            error!("{err:?}");
                        }
                    }
                } else {
                    error!("Node {name:?} was missing either a mesh or a transform");
                }
            }
        }

        let hentai = gltf.animations.clone();

        let mut graph = AnimationGraph::new();
        let hentai_list: Vec<_> = graph
            .add_clips(hentai.into_iter(), 1.0, graph.root)
            .collect();

        let graph = graphs.add(graph);

        if loaded_gltf.config.spawn {
            let mut entity_commands = commands.spawn((
                SceneRoot(first_scene_handle),
                Animation {
                    animation_list: hentai_list.clone(),
                    graph,
                },
                AnimationPlayer::default(),
            ));
            let func = &loaded_gltf.config.entity_initializer;
            func(&mut entity_commands);
        }
        loaded_gltf.processed = true;
    }
}

fn get_mesh_from_gltf_node<'a>(
    node_handle: &Handle<GltfNode>,
    meshes: &'a ResMut<Assets<Mesh>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    nodes: &Res<Assets<GltfNode>>,
) -> Option<&'a Mesh> {
    nodes
        .get(node_handle)
        .and_then(|node| node.mesh.as_ref())
        .and_then(|mesh_handle| gltf_meshes.get(mesh_handle))
        .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
        .and_then(|first_primitive| meshes.get(&first_primitive.mesh))
}

fn get_material_from_gltf_node<'a>(
    node_handle: &Handle<GltfNode>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    nodes: &Res<Assets<GltfNode>>,
) -> Option<Handle<StandardMaterial>> {
    nodes
        .get(node_handle)
        .and_then(|node| node.mesh.as_ref())
        .and_then(|mesh_handle| gltf_meshes.get(mesh_handle))
        .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
        .and_then(|first_primitive| first_primitive.material.clone())
}

// taken from https://github.com/Defernus/bevy_gltf_collider/blob/9f27253e6d2e645c3570bebead34a493e4da1deb/src/mesh_collider.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColliderFromMeshError {
    MissingPositions,
    MissingIndices,
    InvalidIndicesCount(usize),
    InvalidPositionsType(&'static str),
    TrimeshBuilderFailure(),
}

fn get_collider_from_mesh(
    mesh: &Mesh,
    transform: &Transform,
) -> Result<Collider, ColliderFromMeshError> {
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .map_or(Err(ColliderFromMeshError::MissingPositions), Ok)?;

    let indices = mesh
        .indices()
        .map_or(Err(ColliderFromMeshError::MissingIndices), Ok)?;

    let positions = match positions {
        VertexAttributeValues::Float32x3(positions) => positions,
        v => {
            return Err(ColliderFromMeshError::InvalidPositionsType(
                v.enum_variant_name(),
            ));
        }
    };

    let indices: Vec<u32> = match indices {
        Indices::U32(indices) => indices.clone(),
        Indices::U16(indices) => indices.iter().map(|&i| i as u32).collect(),
    };

    if indices.len() % 3 != 0 {
        return Err(ColliderFromMeshError::InvalidIndicesCount(indices.len()));
    }

    let triple_indices = indices.chunks(3).map(|v| [v[0], v[1], v[2]]).collect();
    let vertices = positions
        .iter()
        .map(|v| {
            let p = Vec4::new(v[0], v[1], v[2], 1.0);
            let p_transformed = transform.compute_matrix() * p;
            Vec3::new(
                p_transformed.x / p_transformed.w,
                p_transformed.y / p_transformed.w,
                p_transformed.z / p_transformed.w,
            )
        })
        .collect();

    let collider = Collider::trimesh(vertices, triple_indices);

    if collider.is_ok() {
        return Ok(collider.unwrap());
    }
    Err(ColliderFromMeshError::TrimeshBuilderFailure())
}
