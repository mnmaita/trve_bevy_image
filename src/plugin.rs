use bevy::{
    asset::{AssetPath, LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub struct TrveImagePlugin;

impl Plugin for TrveImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageLoadState>();
        app.add_systems(
            Startup,
            (setup_resources, load_images.after(setup_resources)),
        );
        app.add_systems(
            Update,
            update_image_assets_load_state.run_if(not(image_assets_loaded)),
        );
    }
}

/// Determines the name of the directory (within the `assets` directory) from where images will be loaded.
///
/// By default, this is set to "img".
///
/// Since `AssetServer::load_folder()` is unsupported in web builds, it will only be used as the base
/// directory for the file names in the `ImageAssetList` Resource.
#[derive(Resource)]
pub struct ImageAssetFolder<'a>(AssetPath<'a>);

impl<'a> ImageAssetFolder<'a> {
    const DEFAULT_FOLDER_NAME: &'static str = "img";

    pub fn new(path: impl Into<AssetPath<'a>>) -> Self {
        Self(path.into())
    }
}

impl Default for ImageAssetFolder<'_> {
    fn default() -> Self {
        Self(Self::DEFAULT_FOLDER_NAME.into())
    }
}

impl std::fmt::Display for ImageAssetFolder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// List of assets to be loaded from the directory specified in the `ImageAssetFolder` Resource.
///
/// Should be a list of file names with their extension.
///
/// This works as an override for `ImageAssetFolder` in non-web platforms so, if set,
/// assets will be loaded individually and only from this list.
///
/// In web builds this is the default and the only supported option.
///
/// Example:
///
/// ```
/// app.insert_resource(ImageAssetList::new(
///     [
///         "image1.png",
///         "image2.png",
///         "image3.png",
///     ]
///     .to_vec(),
/// ));
/// ```
#[derive(Resource, Default, Deref)]
pub struct ImageAssetList<'a>(Vec<AssetPath<'a>>);

impl<'a> ImageAssetList<'a> {
    pub fn new(paths: Vec<impl Into<AssetPath<'a>>>) -> Self {
        let asset_paths: Vec<_> = paths.into_iter().map(|path| path.into()).collect();
        Self(asset_paths)
    }
}

#[derive(Resource, Deref)]
pub struct ImageLoadState(RecursiveDependencyLoadState);

impl Default for ImageLoadState {
    fn default() -> Self {
        Self(RecursiveDependencyLoadState::NotLoaded)
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct ImageHandles(Vec<Handle<Image>>);

#[derive(Resource, Default, Deref, DerefMut)]
struct ImageFolderHandle(Handle<LoadedFolder>);

fn setup_resources(mut commands: Commands) {
    commands.init_resource::<ImageAssetFolder>();

    if cfg!(target_family = "wasm") {
        commands.init_resource::<ImageAssetList>();
    }
}

fn load_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_folder: Res<ImageAssetFolder<'static>>,
    image_asset_list: Option<Res<ImageAssetList<'static>>>,
) {
    if cfg!(not(target_family = "wasm")) && image_asset_list.is_none() {
        // TODO: Verify that files in the directory are actually Image handles
        commands.insert_resource(ImageFolderHandle(
            asset_server.load_folder(image_folder.0.clone()),
        ));
        return;
    }

    if let Some(image_asset_list) = image_asset_list {
        let load_image_asset =
            |path| asset_server.load::<Image>(format!("{}/{path}", *image_folder));
        let handles: Vec<_> = image_asset_list.iter().map(load_image_asset).collect();
        commands.insert_resource(ImageHandles(handles));
    }
}

fn update_image_assets_load_state(
    mut image_load_state: ResMut<ImageLoadState>,
    asset_server: Res<AssetServer>,
    image_handles: Option<Res<ImageHandles>>,
    image_folder_handle: Option<Res<ImageFolderHandle>>,
    image_asset_list: Option<Res<ImageAssetList<'static>>>,
) {
    if cfg!(not(target_family = "wasm")) && image_asset_list.is_none() {
        image_load_state.0 =
            asset_server.recursive_dependency_load_state(&image_folder_handle.unwrap().0);
        return;
    }

    if let Some(image_handles) = image_handles {
        let all_loaded = image_handles.iter().all(|handle| {
            if asset_server.recursive_dependency_load_state(handle).is_failed()
            {
                if let Some(path) = handle.path() {
                    info!("Asset '{path}' failed to load. Make sure the file name is correct and is an image.");
                }
                return true;
            }
            asset_server.is_loaded_with_dependencies(handle)
        });

        image_load_state.0 = match all_loaded {
            true => RecursiveDependencyLoadState::Loaded,
            false => RecursiveDependencyLoadState::NotLoaded,
        };
    }
}

pub fn image_assets_loaded(image_load_state: Res<ImageLoadState>) -> bool {
    image_load_state.is_loaded()
}
