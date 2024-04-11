use bevy::{math::vec3, prelude::*, window::PrimaryWindow};
use perlin2d::PerlinNoise2D;
use rand::prelude::*;

const WORLD_X: i32 = 100;
const WORLD_Y: i32 = 100;

#[derive(Component,Debug)]
struct WorldPixel(String);

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component,Debug)]
struct MachineType(String);

fn main() {
    App::new()
        .insert_resource(MyWorldCoords(Vec2 { x: 0., y: 0. }))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_ui, cursor_to_world_position, pixel_interaction, camera_movement))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(TextBundle::from_section("vec2".to_string(), TextStyle {font_size: 60., ..default()} ));
    generate_world(&mut commands);
}

fn generate_world(mut commands: &mut Commands) {
    // octaves: 6, amplitude: 1, frequency: 0.6
    let perlin = PerlinNoise2D::new(6, 1.0, 0.6, 1.0, 2.0, (-120.0, 120.0), 0.5, 101);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while x < WORLD_X {
        while y < WORLD_Y {
            let val = perlin.get_noise(x as f64, y as f64);
            println!("{:?}", val);
            // biome order
            // 1 deep water 2 water  3 sand  4 grass 5 mountain
            if val <= -0.25 { // deep water 
                generate_pixel(&mut commands, Color::rgb(0.1, 0.7, 0.8), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("water0".to_string()));
            }
            if val > -0.25 && val <= -0.5 { // water
                generate_pixel(&mut commands, Color::rgb(0.3, 0.7, 0.9), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("water0".to_string()));
            }
            if val > -0.5 && val <= -0.25 { // sand
                generate_pixel(&mut commands,Color::rgb(0.3, 0.76, 0.9), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("sand0".to_string()));
            }
            if val > -0.25 && val <= 0. { // grass
                generate_pixel(&mut commands,Color::rgb(1., 0.9, 0.5),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("grass0".to_string()));
            }
            if val > 0. && val <= 0.5 { // mountain
                generate_pixel(&mut commands,Color::rgb(0.5, 0.9, 0.4),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("mountain0".to_string()));
            }
            if val > 0.5 { 
                let mut rng = rand::thread_rng();
                let r: f64 = rng.gen();
                if r < 0.92 {
                    generate_pixel(&mut commands,Color::rgb(0.63, 0.64, 0.67), Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("stone0".to_string()));
                } else {
                    generate_pixel(&mut commands,Color::rgb(1., 1., 0.4),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("gold".to_string()));
                }
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }
}

fn generate_pixel(commands: &mut Commands, color:Color, position:Transform, material:WorldPixel) {
    commands.spawn(SpriteBundle {
        sprite: Sprite { color: color, ..default() },
        transform: position,
        ..default()
    }).insert(material);
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for mut p in query.iter_mut() {
        if input.pressed(KeyCode::KeyA) {
            println!("{:?}", p);
            p.translation += vec3(-120., 0., 0.) * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyD) {
            println!("{:?}", p);
            p.translation += vec3(120., 0., 0.) * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyW) {
            println!("{:?}", p);
            p.translation += vec3(0., 120., 0.) * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyS) {
            println!("{:?}", p);
            p.translation += vec3(0., -120., 0.) * time.delta_seconds();
        }

        for mut c in camera_query.iter_mut() {
            if input.pressed(KeyCode::KeyE) {
                c.scale += 0.8 * time.delta_seconds();
                println!("{:?}", c);
            }
            if input.pressed(KeyCode::KeyQ) {
                c.scale += -0.8 * time.delta_seconds();
                println!("{:?}", c);
            }   
        }

    }
}

fn cursor_to_world_position(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

fn update_ui(
    mut mouse_text: Query<&mut Text>,
    mouse_position: Res<MyWorldCoords>
) {
    for mut text in mouse_text.iter_mut() {
        text.sections[0].value = mouse_position.0.to_string();
    }
}

fn pixel_interaction(
    mut world_data: Query<(&mut Sprite, &Transform), With<WorldPixel>>,
    mouse_position: Res<MyWorldCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    if input.pressed(MouseButton::Left) {
        let x_value = mouse_position.0.x as i64;
        let y_value = mouse_position.0.y as i64;
        let click_position = Vec3::new(x_value as f32, y_value as f32, 0.9);

        for pixel in world_data.iter_mut() {
            if pixel.1.translation == click_position {
            //    pixel.0.color = Color::GREEN;
               spawn_machine(&mut commands, &asset_server, click_position);
            }
        }
    }
}

fn spawn_machine(commands: &mut Commands, asset_server: &AssetServer, position: Vec3) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("turtl.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 0.8)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.9),
            ..default()
        },
        MachineType("turtle".to_string())
    ));
}

