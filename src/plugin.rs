use bevy::{
    asset::{AssetPath, LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

const IMAGE_ASSET_FOLDER: &str = "img";

pub struct TrveImagePlugin;

impl Plugin for TrveImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssetFolder>();
        app.init_resource::<ImageLoadState>();
        app.add_systems(Startup, load_images);
        app.add_systems(
            Update,
            update_image_assets_load_state.run_if(not(resource_equals(ImageLoadState::Loaded))),
        );
    }
}

#[derive(Resource)]
pub struct ImageAssetFolder<'a>(AssetPath<'a>);

impl<'a> ImageAssetFolder<'a> {
    pub fn new(path: impl Into<AssetPath<'a>>) -> Self {
        Self(path.into())
    }
}

#[derive(Resource, Default, Deref)]
pub struct ImageAssetList<'a>(Vec<AssetPath<'a>>);

impl<'a> ImageAssetList<'a> {
    pub fn new(path: Vec<impl Into<AssetPath<'a>>>) -> Self {
        Self(
            path.into_iter()
                .map(|path| path.into())
                .collect::<Vec<AssetPath<'a>>>(),
        )
    }
}

impl Default for ImageAssetFolder<'_> {
    fn default() -> Self {
        Self(IMAGE_ASSET_FOLDER.into())
    }
}

#[derive(Default, Resource, PartialEq)]
enum ImageLoadState {
    #[default]
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

impl From<RecursiveDependencyLoadState> for ImageLoadState {
    fn from(value: RecursiveDependencyLoadState) -> Self {
        match value {
            RecursiveDependencyLoadState::NotLoaded => Self::NotLoaded,
            RecursiveDependencyLoadState::Loading => Self::Loading,
            RecursiveDependencyLoadState::Loaded => Self::Loaded,
            RecursiveDependencyLoadState::Failed => Self::Failed,
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct ImageHandles(Vec<Handle<Image>>);

#[derive(Resource, Default, Deref, DerefMut)]
struct ImageFolderHandle(Handle<LoadedFolder>);

fn load_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_folder: Res<ImageAssetFolder<'static>>,
    image_asset_list: Option<Res<ImageAssetList<'static>>>,
) {
    if cfg!(not(target_family = "wasm")) && image_asset_list.is_none() {
        commands.insert_resource(ImageFolderHandle(
            asset_server.load_folder(image_folder.0.clone()),
        ));
        return;
    }

    if let Some(image_asset_list) = image_asset_list {
        if image_asset_list.is_empty() {
            if cfg!(target_family = "wasm") {
                info!("ImageAssetList Resource is empty.");
            }
        } else {
            commands.insert_resource(ImageHandles(
                image_asset_list
                    .iter()
                    .map(|path| asset_server.load::<Image>(path))
                    .collect::<Vec<Handle<Image>>>(),
            ));
        }
    } else if cfg!(target_family = "wasm") {
        warn!("ImageAssetList Resource does not exist.");
    }
}

fn update_image_assets_load_state(
    mut textures_load_state: ResMut<ImageLoadState>,
    asset_server: Res<AssetServer>,
    image_handles: Option<Res<ImageHandles>>,
    image_folder_handle: Option<Res<ImageFolderHandle>>,
    image_asset_list: Option<Res<ImageAssetList<'static>>>,
) {
    if image_asset_list.is_some() {
        if let Some(image_handles) = image_handles {
            let all_loaded = image_handles.iter().all(|handle| {
                asset_server.recursive_dependency_load_state(handle.id())
                    == RecursiveDependencyLoadState::Loaded
            });
            *textures_load_state = if all_loaded {
                RecursiveDependencyLoadState::Loaded.into()
            } else {
                RecursiveDependencyLoadState::NotLoaded.into()
            }
        }
    } else if let Some(image_folder_handle) = image_folder_handle {
        *textures_load_state = asset_server
            .recursive_dependency_load_state(image_folder_handle.clone())
            .into()
    }
}

pub fn image_assets_loaded() -> impl Condition<()> {
    IntoSystem::into_system(resource_equals(ImageLoadState::Loaded))
}
