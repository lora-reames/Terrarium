use bevy::{math::uvec2, prelude::*, window::WindowResolution};
use bevy_simple_tilemap::prelude::*;
use std::ops::Range;

const WIDTH: i32 = 31;
const HEIGHT: i32 = 31;
const X_MAX: i32 = WIDTH / 2;
const X_MIN: i32 = -1 * X_MAX;
const Y_MAX: i32 = HEIGHT / 2;
const Y_MIN: i32 = -1 * Y_MAX;

const X_RANGE: Range<i32> = -(X_MAX)..(X_MAX + 1);
const Y_RANGE: Range<i32> = -(Y_MAX)..(Y_MAX + 1);


#[derive(Resource)]
struct GlobalTimer(Timer);

pub struct TerrariumSetupPlugin;

impl Plugin for TerrariumSetupPlugin {
    fn build(&self, app: &mut App) {
        // Create global timer
        app.insert_resource(GlobalTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

        app.add_systems(Startup, setup);
        app.add_systems(Update, update_time);

        fn setup(
            asset_server: Res<AssetServer>,
            mut commands: Commands,
            mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
        ) {
            commands.spawn(Camera2dBundle::default());

            // Load tilesheet texture and make a texture atlas from it
            let texture = asset_server.load("Fields.png");
            let atlas = TextureAtlasLayout::from_grid(uvec2(16, 16), 6, 7, None, None);
            let texture_atlas = texture_atlases.add(atlas);

            // List to store set tile operations
            let mut tiles: Vec<(IVec3, Option<Tile>)> =
                Vec::with_capacity((WIDTH * HEIGHT) as usize);

            // fill background layer
            for y in Y_RANGE {
                for x in X_RANGE {
                    // match spite index
                    let sprite_index: u32 = match (x, y) {
                        // corners
                        (x, y) if x == X_MIN && y == Y_MAX => 18, // Top Left
                        (x, y) if x == X_MAX && y == Y_MAX => 20, // Top Right
                        (x, y) if x == X_MIN && y == Y_MIN => 30, // Bottom Left
                        (x, y) if x == X_MAX && y == Y_MIN => 32, // Bottom Right

                        // edges
                        (.., y) if y == Y_MAX => 19, // Top Edge
                        (x, ..) if x == X_MIN => 24, // Left Edge
                        (x, ..) if x == X_MAX => 26, // Right Edge
                        (.., y) if y == Y_MIN => 31, // Bottom Edge

                        // fill
                        _ => 25,
                    };
                    // Add tile change to list
                    tiles.push((
                        IVec3::new(x, y, 0),
                        Some(Tile {
                            sprite_index,
                            ..Default::default()
                        }),
                    ));
                }
            }

            let mut tilemap = TileMap::default();
            tilemap.set_tiles(tiles);

            // Set up tilemap
            let tilemap_bundle = TileMapBundle {
                tilemap,
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas,
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::splat(2.0),
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            };
            // Spawn tilemap
            commands.spawn(tilemap_bundle);
        }

        fn update_time(time: Res<Time>, mut timer: ResMut<GlobalTimer>) {
            if timer.0.tick(time.delta()).just_finished() {
                println!("tick! the elapsed time is now {:?}", time.elapsed())
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.8, 0.69, 0.38)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Terrarium".to_string(),
                        resolution: WindowResolution::new(1024.0, 1024.0)
                            .with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleTileMapPlugin)
        .add_plugins(TerrariumSetupPlugin)
        .run();
}