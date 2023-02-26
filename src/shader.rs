use std::borrow::Cow;

use bevy::{
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  reflect::TypeUuid,
  render::{
    mesh::MeshVertexBufferLayout,
    render_resource::{
      AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
    },
  },
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
#[bind_group_data(ShaderMaterialKey)]
pub struct ShaderMaterial {
  pub entry_point: Cow<'static, str>,
}

impl Material for ShaderMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/custom.wgsl".into()
  }

  fn alpha_mode(&self) -> AlphaMode {
    AlphaMode::Opaque
  }

  fn specialize(
    _pipeline: &MaterialPipeline<Self>,
    descriptor: &mut RenderPipelineDescriptor,
    _layout: &MeshVertexBufferLayout,
    key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let fragment = descriptor.fragment.as_mut().unwrap();
    fragment.entry_point = key.bind_group_data.entry_point.clone();
    Ok(())
  }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ShaderMaterialKey {
  entry_point: Cow<'static, str>,
}

impl From<&ShaderMaterial> for ShaderMaterialKey {
  fn from(value: &ShaderMaterial) -> Self {
    Self {
      entry_point: value.entry_point.clone(),
    }
  }
}
