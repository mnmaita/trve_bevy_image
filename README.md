# trve_bevy_image

An opinionated Bevy plugin to load Image Assets quickly and easily. Ideal for Game Jams.

## How to use

This plugin is meant to be a convenience tool to load all image assets for your game at startup, ideal for small projects and prototypes.

To use it, add it to your Cargo.toml file like this:

```toml
trve_bevy_image = { git = "https://github.com/mnmaita/trve_bevy_image" }
```

Remember you can also target tags, commits and branches with this method:

```toml
trve_bevy_image = { git = "https://github.com/mnmaita/trve_bevy_image", tag = "v0.6.0" }
```

```toml
trve_bevy_image = { git = "https://github.com/mnmaita/trve_bevy_image", branch = "test" }
```

```toml
trve_bevy_image = { git = "https://github.com/mnmaita/trve_bevy_image", rev = "some-sha" }
```

### Default usage and overriding default behavior

By default, it will load all assets from an "img" directory under your "assets" folder. You can override this directory by using the `ImageAssetFolder` Resource:

```rs
let mut app = App::new();

// Your plugins go here.
app.add_plugins(TrveImagePlugin);

// You insert this Resource and use the `new` function
// which accepts any parameter that can be turned into an `AssetPath`.
app.insert_resource(ImageAssetFolder::new("pngs"));
```

This will load all assets from `assets/pngs` by using `AssetServer`'s `load_folder` method.

### Loading a list of assets

Certain platforms, like web, can't use `load_folder` to load assets so this library provides an override via the `ImageAssetList` Resource.

This allows you to load a list of assets from the folder specified in the `ImageAssetFolder` Resource, within the `assets` directory.

```rust
    // This will attempt to load `assets/img/texture1.png`, `assets/img/texture2.png` and `assets/img/texture3.png`.
    app.insert_resource(ImageAssetList::new(
        [
            "texture1.png",
            "texture2.png",
            "texture3.png",
        ]
        .into(),
    ));
```

```rust
    // This will attempt to load `assets/pngs/texture1.png`, `assets/pngs/texture2.png` and `assets/pngs/texture3.png`.
    app.insert_resource(ImageAssetFolder::new("pngs"));
    app.insert_resource(ImageAssetList::new(
        [
            "texture1.png",
            "texture2.png",
            "texture3.png",
        ]
        .into(),
    ));
```

If you insert this Resource the plugin will **only** load the assets provided in the list.

## Bevy version compatibility

| trve_bevy_image | bevy |
| --------------- | ---- |
| 0.6             | 0.16 |
| 0.5             | 0.15 |
| 0.3, 0.4        | 0.14 |
| 0.2             | 0.13 |
| 0.1             | 0.12 |
