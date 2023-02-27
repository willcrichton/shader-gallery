use bevy::{
  core_pipeline::clear_color::ClearColorConfig,
  prelude::*,
  render::{
    camera::RenderTarget,
    render_resource::{
      Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    view::RenderLayers,
  },
};

use crate::environment::EnvState;

use self::simple::SimpleScene;

mod simple;

const HIDDEN_LAYER: RenderLayers = RenderLayers::layer(1);
// const ASPECT_RATIO: f32 = 0.1736;

fn create_painting(
  commands: &mut Commands,
  images: &mut Assets<Image>,
  transform: Transform,
) -> Handle<Image> {
  let size = Extent3d {
    width: 512,
    height: 512,
    ..default()
  };

  // This is the texture that will be rendered to.
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
    },
    ..default()
  };
  image.resize(size);

  let image_handle = images.add(image);

  commands.spawn((
    Camera3dBundle {
      camera_3d: Camera3d {
        clear_color: ClearColorConfig::Custom(Color::WHITE),
        ..default()
      },
      camera: Camera {
        // render before the "main pass" camera
        priority: -1,
        target: RenderTarget::Image(image_handle.clone()),
        ..default()
      },
      transform,
      ..default()
    },
    HIDDEN_LAYER,
  ));

  image_handle
}

pub fn create_paintings(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  query: Query<(&Name, &Handle<Mesh>, &GlobalTransform)>,
) {
  let paintings = [("paintingMid3", SimpleScene::camera_transform())];

  for (painting_name, transform) in paintings {
    let image_handle = create_painting(&mut commands, &mut images, transform);

    let material = materials.add(StandardMaterial {
      base_color_texture: Some(image_handle),
      reflectance: 0.02,
      unlit: false,
      ..default()
    });

    let (_, mesh_handle, global_transform) = query
      .iter()
      .find(|(name, ..)| name.as_str() == painting_name)
      .unwrap();

    let mesh: &Mesh = meshes.get(mesh_handle).unwrap();
    let aabb = mesh.compute_aabb().unwrap();
    let transform = global_transform.compute_transform();
    let transform = transform.with_translation(aabb.center.into());

    commands.spawn(PbrBundle {
      mesh: meshes.add(
        shape::Box::new(
          aabb.half_extents.x * 2.,
          aabb.half_extents.y * 2.,
          aabb.half_extents.z * 2.,
        )
        .into(),
      ),
      transform,
      material: material.clone(),
      ..default()
    });
  }
}

pub struct PaintingsPlugin;
impl Plugin for PaintingsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(simple::SimpleScene)
      .add_system_set(SystemSet::on_enter(EnvState::Ready).with_system(create_paintings));
  }
}
