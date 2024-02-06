use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Add a camera that will always fit the virtual resolution WIDTH x HEIGHT
    // inside the window.
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSmallerDim(HEIGHT),
        PixelViewport,
    ));

    let mire_handle = asset_server.load("mire-64x64.png");

    let texture_atlas =
        TextureAtlas::from_grid(mire_handle.clone(), Vec2::new(32.0, 32.0), 2, 2, None, None);
        
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add a mire sprite in the center of the window.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });

    // Add a mire sprite in the bottom-left corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            -(WIDTH / 2) as f32,
            -(HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });

    // Add a mire sprite in the bottom-right corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            (WIDTH / 2) as f32,
            -(HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });

    // Add a mire sprite in the top-left corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            -(WIDTH / 2) as f32,
            (HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });

    // Add a mire sprite in the top-right corner of our virtual resolution.
    // commands.spawn(SpriteSheetBundle {
    //     texture_atlas: texture_atlas_handle.clone(),
    //     transform: Transform::from_translation(Vec3::new(
    //         (WIDTH / 2) as f32 - 32.,
    //         (HEIGHT / 2) as f32 - 32.,
    //         0.0,
    //     )),
    //     sprite: TextureAtlasSprite{index: 1, anchor: bevy::sprite::Anchor::BottomLeft, ..Default::default()},
    //     ..Default::default()
    // });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            rect: Some(Rect{
                min: Vec2{x: 0., y: 0.},
                max: Vec2{x: 32., y: 32.},
            }),
            ..Default::default()
        },
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            (WIDTH / 2) as f32,
            (HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });
}