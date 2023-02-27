use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_fbx::FbxPlugin;
use environment::EnvState;
use shader::ShaderMaterial;
use smooth_bevy_cameras::{
  controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
  LookTransformPlugin,
};

mod environment;
mod paintings;
mod shader;

fn main() {
  let mut app = App::new();

  app
    .insert_resource(AmbientLight {
      color: Color::WHITE,
      brightness: 1.0 / 2.0f32,
    })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        fit_canvas_to_parent: true,
        ..default()
      },
      ..default()
    }))
    .add_plugin(MaterialPlugin::<ShaderMaterial>::default())
    .add_plugin(LookTransformPlugin)
    .add_plugin(FpsCameraPlugin::default())
    .add_plugin(WorldInspectorPlugin)
    .add_plugin(FbxPlugin)
    .add_plugin(environment::EnvironmentPlugin)
    .add_plugin(paintings::PaintingsPlugin)
    .add_system_set(SystemSet::on_enter(EnvState::Ready).with_system(on_load));

  app.run();
}

fn on_load(mut commands: Commands) {
  let mut camera = Camera3dBundle::default();
  let Projection::Perspective(perspective) = &mut camera.projection else { unreachable!() };
  perspective.fov = PI / 4. * 1.2;
  commands.spawn(camera).insert(FpsCameraBundle::new(
    FpsCameraController::default(),
    Vec3::new(0., 1.8, 0.),
    Vec3::new(2., 1.8, 2.),
    Vec3::Y,
  ));
}
