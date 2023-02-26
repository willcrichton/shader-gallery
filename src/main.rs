use bevy::{
  core_pipeline::core_3d::{AlphaMask3d, Opaque3d, Transparent3d},
  ecs::system::{lifetimeless::SRes, SystemParamItem},
  prelude::*,
  render::{
    render_asset::RenderAssets,
    render_phase::{
      AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommand, RenderCommandResult,
      RenderPhase, TrackedRenderPass,
    },
    render_resource::{AsBindGroup, AsBindGroupShaderType, PreparedBindGroup},
    renderer::RenderDevice,
    texture::FallbackImage,
    view::VisibleEntities,
    RenderApp, RenderStage,
  },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shader::ShaderMaterial;
use smooth_bevy_cameras::{
  controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
  LookTransformPlugin,
};

mod shader;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum WorldState {
  Loading,
  Playing,
}

#[derive(Resource, Default)]
struct Environment {
  handle: Handle<Scene>,
}

fn main() {
  let mut app = App::new();

  app
    .insert_resource(AmbientLight {
      color: Color::WHITE,
      brightness: 1.0 / 2.0f32,
    })
    .init_resource::<Environment>()
    .add_state(WorldState::Loading)
    .add_plugins(DefaultPlugins)
    .add_plugin(MaterialPlugin::<ShaderMaterial>::default())
    .add_plugin(LookTransformPlugin)
    .add_plugin(FpsCameraPlugin::default())
    .add_plugin(WorldInspectorPlugin)
    .add_plugin(bevy_mod_fbx::FbxPlugin)
    .add_startup_system(setup)
    .add_system(update_uniforms)
    .add_system_set(SystemSet::on_update(WorldState::Loading).with_system(wait_for_load))
    .add_system_set(SystemSet::on_enter(WorldState::Playing).with_system(on_load));

  let render_app = app.get_sub_app_mut(RenderApp).unwrap();
  render_app
    .add_render_command::<Opaque3d, ConfigureShaders>()
    .add_system_to_stage(RenderStage::Prepare, render_setup)
    .add_system_to_stage(RenderStage::PhaseSort, render);

  app.run();
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut environment: ResMut<Environment>,
) {
  commands
    .spawn(Camera3dBundle::default())
    .insert(FpsCameraBundle::new(
      FpsCameraController::default(),
      Vec3::new(0., 1.8, 0.),
      Vec3::new(2., 1.8, 2.),
      Vec3::Y,
    ));

  // "models/floorTest.gltf#Scene0"
  environment.handle = asset_server.load("models/art_gallery/scene.gltf#Scene0");
  commands.spawn(SceneBundle {
    scene: environment.handle.clone(),
    ..default()
  });
}

fn render_setup(
  mut commands: Commands,
  render_device: Res<RenderDevice>,
  images: Res<RenderAssets<Image>>,
  fallback_image: Res<FallbackImage>,
  mut setup: Local<bool>,
) {
  if *setup {
    return;
  }

  let uniforms = CommonUniforms { time: 0. };
  let bind_group = uniforms
    .as_bind_group(
      &CommonUniforms::bind_group_layout(&render_device),
      &render_device,
      &images,
      &fallback_image,
    )
    .map_err(|_| "Could not create bind group")
    .unwrap();
  commands.insert_resource(CommonUniformsBindGroup(bind_group));
  *setup = true;
}

fn update_uniforms(uniforms: ResMut<CommonUniformsBindGroup>) {
    uniforms.0.data.time += 0.03;
}

fn on_load(
  mut commands: Commands,
  //   query: Query<(Entity, &Name, &Handle<Mesh>, &GlobalTransform)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ShaderMaterial>>,
) {
  for i in 0..2 {
    commands.spawn(MaterialMeshBundle {
      mesh: meshes.add(shape::Box::new(1., 1., 0.1).into()),
      transform: Transform::from_translation(Vec3::new(2. * i as f32, 1., 1.)),
      material: materials.add(shader::ShaderMaterial {
        // time: 0,
        entry_point: format!("fragment{i}").into(),
      }),
      ..default()
    });
  }

  //   println!("Loaded!");
  //   for (entity, name, mesh_handle, global_transform) in &query {
  //     // eprintln!("{name} {:#?}", global_transform.compute_transform());
  //     if &**name == "PaitingsInside_Painting_0" {
  //       let mesh: &Mesh = meshes.get(mesh_handle).unwrap();
  //       println!("{}", mesh.count_vertices());
  //       eprintln!("{:#?}", mesh.indices());

  //       //   if let Some(VertexAttributeValues::Float32x3(values)) =
  //       //     mesh.attribute(Mesh::ATTRIBUTE_POSITION)
  //       //   {
  //       //     eprintln!("{values:#?}");
  //       //   }
  //       //   println!("{:#?}", global_transform.to_scale_rotation_translation());
  //       //   println!("{:#?}", global_transform.compute_transform());

  //       //   commands.spawn(PbrBundle {
  //       //     mesh: meshes.add(shape::Box::new(1., 1., 0.1).into()),
  //       //     transform: global_transform.compute_transform(),
  //       //     material: materials.add(Color::AQUAMARINE.into()),
  //       //     ..default()
  //       //   });
  //     }
  //   }
}

#[derive(AsBindGroup)]
pub struct CommonUniforms {
  #[uniform(0)]
  pub time: f32,
}

#[derive(Resource)]
pub struct CommonUniformsBindGroup(PreparedBindGroup<CommonUniforms>);

pub struct SetShaderSharedUniforms<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetShaderSharedUniforms<I> {
  type Param = SRes<CommonUniformsBindGroup>;

  fn render<'w>(
    view: Entity,
    item: Entity,
    bind_group: SystemParamItem<'w, '_, Self::Param>,
    pass: &mut TrackedRenderPass<'w>,
  ) -> bevy::render::render_phase::RenderCommandResult {
    let bind_group = &bind_group.into_inner().0.bind_group;
    pass.set_bind_group(I, bind_group, &[]);
    RenderCommandResult::Success
  }
}

type ConfigureShaders = (SetShaderSharedUniforms<3>,);

fn wait_for_load(
  mut asset_events: EventReader<AssetEvent<Scene>>,
  mut state: ResMut<State<WorldState>>,
) {
  for event in asset_events.iter() {
    match event {
      AssetEvent::Created { .. } => state.set(WorldState::Playing).unwrap(),
      _ => {}
    }
  }
}

fn render(
  mut views: Query<&mut RenderPhase<Opaque3d>>,
  opaque_draw_functions: Res<DrawFunctions<Opaque3d>>,
) {
  let draw_function = opaque_draw_functions
    .read()
    .get_id::<ConfigureShaders>()
    .unwrap();
  for mut opaque_phase in &mut views {
    take_mut::take(&mut opaque_phase.items, |items| {
      items
        .into_iter()
        .flat_map(|item| {
          [
            Opaque3d {
              entity: item.entity,
              distance: item.distance,
              pipeline: item.pipeline,
              draw_function,
            },
            item,
          ]
        })
        .collect()
    });
  }
}
