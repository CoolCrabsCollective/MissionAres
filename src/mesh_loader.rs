use crate::{
    hentai_anime::Animation,
    scene_hook::{SceneHook, run_hooks},
};
use bevy::{
    gltf::{Gltf, GltfMaterialName, GltfMesh, GltfNode},
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
    scene::scene_spawner_system,
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
    pub entity_initializer: Option<Box<dyn Fn(&mut EntityCommands) + Send + Sync>>,
    pub scene_color_override: Option<Color>,
    /// material
    // pub material_initializer: Option<
    //     Box<dyn Fn(Handle<StandardMaterial>, &mut StandardMaterial, Option<Entity>) + Send + Sync>,
    // >,
    // pub material_replacer: Option<Box<dyn Fn(&StandardMaterial) -> StandardMaterial + Send + Sync>>,
    /// Whether to generate a (static, non rigid body) collider for the loaded GLTF during load
    pub generate_static_collider: bool,
    /// CollisionGroups to use for the generated collider
    pub collision_groups: CollisionGroups,
}

impl Default for GLTFLoadConfig {
    fn default() -> Self {
        Self {
            spawn: true,
            entity_initializer: None,
            scene_color_override: None,
            // material_initializer: None,
            // material_replacer: None,
            generate_static_collider: false,
            collision_groups: CollisionGroups::default(),
        }
    }
}

pub struct LoadedGLTF {
    pub gltf_handle: Handle<Gltf>,
    pub config: GLTFLoadConfig,
    pub processed: bool,
    pub file_path: String,
}

#[derive(Resource)]
pub struct MeshLoader(pub(crate) Vec<LoadedGLTF>);

#[derive(Component)]
pub struct SceneColorOverride(pub Option<(Handle<Scene>, Color)>);

#[derive(Event)]
pub struct DebugLogEntityRequest(pub Entity);

impl Plugin for MeshLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, process_loaded_gltfs.after(setup));
        app.add_systems(SpawnScene, run_hooks.after(scene_spawner_system));
        app.add_systems(Update, apply_scene_color_override);
        app.add_event::<DebugLogEntityRequest>();
        app.add_systems(Update, on_debug_log_entity_request);
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
        gltf_handle: asset_server.load(asset_path.clone()),
        processed: false,
        config,
        file_path: asset_path,
    });
}

#[allow(clippy::too_many_arguments)]
fn process_loaded_gltfs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gltf_meshes: ResMut<Assets<GltfMesh>>,
    mut nodes: ResMut<Assets<GltfNode>>,
    mut mesh_loader: ResMut<MeshLoader>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

        let asset_path = &loaded_gltf.file_path;

        let mut spawned_entity = None;

        // let material_replacer = loaded_gltf.config.material_replacer.take();

        if loaded_gltf.config.spawn {
            let mut entity_commands = commands.spawn((
                SceneRoot(first_scene_handle.clone()),
                Animation {
                    animation_list: vec![],
                    graph: graphs.reserve_handle(),
                    group_is_playing: false,
                },
            ));

            if let Some(scene_color_override) = loaded_gltf.config.scene_color_override {
                entity_commands.insert(SceneHook::new(move |_entity, cmds| {
                    // log::info!(
                    //     "Queuing scene color override for scene {:?}",
                    //     first_scene_handle
                    // );

                    cmds.insert(SceneColorOverride(Some((
                        first_scene_handle.clone(),
                        scene_color_override,
                    ))));
                }));
            }
            // world.inspect_entity(entity.id());
            // if entity.contains::<MeshMaterial3d<StandardMaterial>>() {
            // cmds.insert(ReplaceMyMaterial(material_replacer));
            // if let Some(material_replacer) = &material_replacer {
            //     if let Some(material_handle) =
            //         entity.get::<MeshMaterial3d<StandardMaterial>>()
            //     {
            //         if let Some(material) = materials.get(material_handle) {
            //             let material = material_replacer(material);
            //             cmds.insert(MeshMaterial3d(materials.add(material)));
            //         }
            //     }
            // }
            // }
            // }));
            // }

            if let Some(entity_initializer) = &loaded_gltf.config.entity_initializer {
                entity_initializer(&mut entity_commands);
            }
            spawned_entity = Some(entity_commands.id());
        }

        for (name, node_handle) in &gltf.named_nodes {
            if let (Some(mesh), Some(material_handle), Some(transform)) = (
                get_mesh_from_gltf_node(node_handle, &meshes, &gltf_meshes, &nodes),
                get_material_from_gltf_node(node_handle, &gltf_meshes, &nodes),
                nodes.get(node_handle).map(|node| node.transform),
            ) {
                // if let Some(material_initializer) = &loaded_gltf.config.material_initializer {
                //     log::info!(
                //         "Initializing material for node {name:?} and material {:?}",
                //         material_handle
                //     );
                //     let material = materials.get_mut(&material_handle).unwrap();
                //     material_initializer(material_handle, material, spawned_entity);
                // }

                // prepend the node name to the material name, if any
                // if let Some(primitive) = nodes
                //     .get_mut(node_handle)
                //     .and_then(|node| node.mesh.as_ref())
                //     .and_then(|mesh_handle| gltf_meshes.get_mut(mesh_handle))
                //     .and_then(|gltf_mesh| gltf_mesh.primitives.get_mut(0))
                // {
                //     log::info!(
                //         "Prepending node name to mesh primitive name: {name:?}, {}",
                //         primitive.name
                //     );
                //     primitive.name = format!("{}_{}", name, primitive.name);
                // }

                if loaded_gltf.config.generate_static_collider {
                    info!("Generating collider from level object: {name:?}");

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
                }
            } else if loaded_gltf.config.generate_static_collider {
                error!("Node {name:?} was missing either a mesh or a transform");
            }
        }

        loaded_gltf.processed = true;
    }
}

fn get_mesh_from_gltf_node<'a>(
    node_handle: &Handle<GltfNode>,
    meshes: &'a ResMut<Assets<Mesh>>,
    gltf_meshes: &ResMut<Assets<GltfMesh>>,
    nodes: &ResMut<Assets<GltfNode>>,
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
    gltf_meshes: &ResMut<Assets<GltfMesh>>,
    nodes: &ResMut<Assets<GltfNode>>,
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

fn apply_scene_color_override(
    mut commands: Commands,
    mut debug_log_entity_request: EventWriter<DebugLogEntityRequest>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_color_override: Query<(
        Entity,
        &mut SceneColorOverride,
        Option<&SceneRoot>,
        Option<&MeshMaterial3d<StandardMaterial>>,
        Option<&Name>,
        Option<&GltfMaterialName>,
        Option<&Children>,
    )>,
) {
    for (
        entity,
        mut scene_color_override,
        scene_root,
        material,
        name,
        gltf_material_name,
        children,
    ) in scene_color_override
    {
        let Some((scene_handle, new_color)) = scene_color_override.0.take() else {
            continue;
        };

        // debug_log_entity_request.write(DebugLogEntityRequest(entity));

        // also forward the scene color override to all children of the entity
        if let Some(children) = children {
            for child in children.iter() {
                commands
                    .entity(child)
                    .insert(SceneColorOverride(Some((scene_handle.clone(), new_color))));
            }
        }

        let Some(material) = material else {
            continue;
        };

        let Some(existing_material) = materials.get(material).cloned() else {
            continue;
        };

        let Some(GltfMaterialName(gltf_material_name)) = gltf_material_name else {
            continue;
        };

        if !gltf_material_name.contains("col") {
            continue;
        }

        // log::info!(
        //     "Applying scene color override for scene {:?} ({:?}) to material {:?} ({:?})",
        //     scene_handle,
        //     scene_root,
        //     material,
        //     name
        // );
        let new_material_handle = materials.add(StandardMaterial {
            base_color: new_color,
            ..existing_material.clone()
        });
        commands
            .entity(entity)
            .insert(MeshMaterial3d(new_material_handle));
    }
}

fn on_debug_log_entity_request(mut events: EventReader<DebugLogEntityRequest>, world: &World) {
    for event in events.read() {
        log::info!("Printing components for entity: {}", event.0);
        if let Ok(components) = world.inspect_entity(event.0) {
            for component in components {
                log::info!("Component: {}", component.name());
            }
        }
    }
}
