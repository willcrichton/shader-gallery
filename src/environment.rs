use bevy::prelude::*;

#[derive(Resource, Default)]
struct Environment {
  handle: Handle<Scene>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum EnvState {
  Loading,
  Ready,
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut environment: ResMut<Environment>,
) {
  // "models/art_gallery/scene.gltf#Scene0"
  environment.handle = asset_server.load("models/shaderHall/shaderHall.gltf#Scene0");
  commands.spawn(SceneBundle {
    scene: environment.handle.clone(),
    ..default()
  });
}

fn wait_for_load(
  mut asset_events: EventReader<AssetEvent<Scene>>,
  mut state: ResMut<State<EnvState>>,
) {
  for event in asset_events.iter() {
    if let AssetEvent::Created { .. } = event {
      state.set(EnvState::Ready).unwrap();
    }
  }
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state(EnvState::Loading)
      .init_resource::<Environment>()
      .add_system_set(SystemSet::on_update(EnvState::Loading).with_system(wait_for_load))
      .add_startup_system(setup);
  }
}
