/// Default modules:
mod sections;
mod asset_file;

/// Generated modules:
mod light;
mod modelimporter;
mod occlusioncullingsettings;
mod transform;
mod prefab;
mod nativeformatimporter;
mod rendersettings;
mod lightmapsettings;
mod navmeshsettings;
mod textureimporter;
mod material;
mod gameobject;
mod meshfilter;
mod meshrenderer;

pub mod prelude {
	pub use super::sections::Vec2;
	pub use super::sections::Vec3;
	pub use super::sections::Vec4;
	pub use super::sections::Color;
	pub use super::sections::Empty;
	pub use super::sections::FileReference;
	pub use super::sections::FalloffTable;
	pub use super::asset_file::AssetFile;
	pub use super::sections::BuildTarget;

	pub use super::light::Light;
	pub use super::modelimporter::ModelImporter;
	pub use super::occlusioncullingsettings::OcclusionCullingSettings;
	pub use super::transform::Transform;
	pub use super::prefab::Prefab;
	pub use super::nativeformatimporter::NativeFormatImporter;
	pub use super::rendersettings::RenderSettings;
	pub use super::lightmapsettings::LightmapSettings;
	pub use super::navmeshsettings::NavMeshSettings;
	pub use super::textureimporter::TextureImporter;
	pub use super::material::Material;
	pub use super::gameobject::GameObject;
	pub use super::meshfilter::MeshFilter;
	pub use super::meshrenderer::MeshRenderer;
}

