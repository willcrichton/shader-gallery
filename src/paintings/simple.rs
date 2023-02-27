use std::f32::consts::PI;

use bevy::prelude::*;

use crate::shader::ShaderMaterial;

use super::HIDDEN_LAYER;

#[derive(Component)]
struct Cube;

fn create_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut shader_materials: ResMut<Assets<ShaderMaterial>>,
) {
  let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
  let material = shader_materials.add(ShaderMaterial {
    entry_point: "fragment1".into(),
  });
  commands.spawn((
    MaterialMeshBundle {
      mesh: cube_handle,
      material,
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
      ..default()
    },
    Cube,
    HIDDEN_LAYER,
  ));
}

fn update_scene(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
  for mut transform in &mut query {
    transform.rotate(Quat::from_axis_angle(
      Vec3::new(1., 1., 0.1).normalize(),
      PI / 2. * time.delta_seconds(),
    ));
  }
}

pub struct SimpleScene;

impl SimpleScene {
  pub fn camera_transform() -> Transform {
    Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)).looking_at(Vec3::ZERO, Vec3::Y)
  }
}

impl Plugin for SimpleScene {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(create_scene)
      .add_system(update_scene);
  }
}
