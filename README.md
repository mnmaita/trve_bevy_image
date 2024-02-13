# trve_bevy_image

An opinionated Bevy plugin to load Image Assets quickly and easily. Ideal for Game Jams.

## How to use

This plugin is meant to be a convenience tool to load all image assets for your game at startup, ideal for small projects and prototypes.

### Default usage and overriding default behavior

By default, it will load all assets from an "img" directory under your "assets" folder. You can override this directory by using the `ImageAssetFolder` Resource:

```rs
let mut app = App::new();

// Your plugins go here.
app.add_plugins(TrveImagePlugin);

// You insert this Resource and use the `new` function
// which accepts any parameter that can be turned into an `AssetPath`.
app.insert_resource(ImageAssetFolder::new("images"));
```

This will load all assets from `assets/images` by using `AssetServer`'s `load_folder` method.

### Loading a list of assets

Certain platforms, like web, can't use `load_folder` to load assets so this library provides an override via the `ImageAssetList` Resource. This allows you to load a list of assets from your `assets` folder.

```rs
    app.insert_resource(ImageAssetList::new(
        [
            "textures/player.png",
            "textures/enemy.png",
            "textures/background.png",
        ]
        .into(),
    ));
```

If you insert this Resource, `ImageAssetFolder` will be ignored and the plugin will only load assets based on the provided list.
