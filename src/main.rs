use bevy::{math::vec3, prelude::*, scene::ron::de::Position, window::PrimaryWindow};
// use perlin2d::PerlinNoise2D;
// use rand::prelude::*;

mod world;
use world::WorldPlugin;

// const WORLD_X: i32 = 10;
// const WORLD_Y: i32 = 10;

#[derive(Component,Debug)]
struct WorldPixel(String);

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component,Debug)]
struct EntityType(String);

#[derive(Resource, Debug)]
struct PixelsTaked(Vec<Vec3>);

fn main() {
    App::new()
        .insert_resource(MyWorldCoords(Vec2 { x: 0., y: 0. }))
        .insert_resource(PixelsTaked(vec![Vec3 { x: 0., y: 0., z: 0. }]))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (cursor_to_world_position, pixel_selected, pixel_interaction, camera_movement))
        .run();
    // startup: select_machine_ui
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(SpriteBundle {
        sprite: Sprite { color: Color::rgba(0.94, 0.94,0.94, 0.3), ..default() },
        transform: Transform::from_xyz(-2., 0., 0.8),
        ..default()
    }).insert(EntityType("selected".to_string()));
    //generate_world(&mut commands);
}

// fn generate_world(mut commands: &mut Commands) {
//     let mut rng = rand::thread_rng();
//     let mut random_value: f64 = rng.gen();
//     // octaves: 6, amplitude: 1, frequency: 0.6
//     let perlin = PerlinNoise2D::new(7, 1.0, 0.6, 1.0, 2.0, (-600.0, 600.0), random_value, 101);

//     let mut x: i32 = 0;
//     let mut y: i32 = 0;
//     while x < WORLD_X {
//         while y < WORLD_Y {
//             let val = perlin.get_noise(x as f64, y as f64);
//             println!("{:?}", val);
//             // biome order
//             // 1 deep water 2 water  3 sand  4 grass 5 mountain
//             random_value = rng.gen();
//             if val <= -0.25 { // deep water 
//                 generate_pixel(&mut commands, Color::rgb(0.1, 0.7, 0.8), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("water0".to_string()));
//             }
//             if val > -0.25 && val <= -0.5 { // water
//                 generate_pixel(&mut commands, Color::rgb(0.3, 0.7, 0.9), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("water0".to_string()));
//             }
//             if val > -0.5 && val <= -0.25 { // sand
//                 generate_pixel(&mut commands,Color::rgb(0.3, 0.76, 0.9), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("sand0".to_string()));
//             }
//             if val > -0.25 && val <= 0. { // grass
//                 generate_pixel(&mut commands,Color::rgb(1., 0.9, 0.5),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("grass0".to_string()));
//             }
//             if val > 0. && val <= 0.5 { // mountain
//                 generate_pixel(&mut commands,Color::rgb(0.5, 0.9, 0.4),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("mountain0".to_string()));
//             }
//             if val > 0.5 { 
//                 if random_value < 0.92 {
//                     generate_pixel(&mut commands,Color::rgb(0.63, 0.64, 0.67), Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("stone0".to_string()));
//                 } else {
//                     generate_pixel(&mut commands,Color::rgb(1., 1., 0.4),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("gold".to_string()));
//                 }
//             }
//             y += 1;
//         }
//         y = 0;
//         x += 1;
//     }
// }

// fn generate_pixel(commands: &mut Commands, color:Color, position:Transform, material:WorldPixel) {
//     commands.spawn(SpriteBundle {
//         sprite: Sprite { color: color, ..default() },
//         transform: position,
//         ..default()
//     }).insert(material);
// }

// fn select_machine_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(NodeBundle {
//         style: Style{
//             display: Display::Flex,
//             width: Val::Percent(100.), 
//             height: Val::Percent(100.),
//             ..default()
//         },
//         ..default()
//     }).with_children(|parent | {
//         parent.spawn(NodeBundle{
//             style: Style { 
//                 display: Display::Flex,
//                 width: Val::Percent(80.), 
//                 height: Val::Percent(100.), 
//                 ..default() 
//             },
//             ..default()
//         }).with_children(|parent| {
//             parent.spawn(TextBundle::from_section("None", TextStyle{font_size:40., color: Color::rgb(0.4, 0.9, 0.4), ..default()}));
//         });
//     }).with_children(|parent | {
//         parent.spawn(NodeBundle{
//             style: Style { 
//                 display: Display::Flex,
//                 flex_direction: FlexDirection::Column,
//                 align_items: AlignItems::Center,
//                 width: Val::Percent(20.), 
//                 height: Val::Percent(100.), 
//                 ..default() 
//             },
//             background_color: BackgroundColor(Color::rgba(0.245,0.223,0.15, 0.180)),
//             ..default()
//         }).with_children(|parent | {
//             parent.spawn(ButtonBundle{
//                 style: Style {
//                     width: Val::Px(200.),
//                     height: Val::Px(150.),
//                     display: Display::Flex,
//                     justify_content: JustifyContent::Center,
//                     align_content: AlignContent::Center,
//                     border: UiRect::all(Val::Px(5.0)),
//                     ..default()
//                 },
//                 image: UiImage{
//                     texture: asset_server.load("turtl.png"),
//                     ..default()
//                 },
//                 background_color: BackgroundColor(Color::rgba(0.89, 0.89, 0.89, 0.5)),
//                 ..default()
//             });
//         });
//     });
// }

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for mut p in query.iter_mut() {
        if input.pressed(KeyCode::KeyA) {
            p.translation += vec3(-120., 0., 0.) * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyD) {
            p.translation += vec3(120., 0., 0.) * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyW) {
            p.translation += vec3(0., 120., 0.) * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyS) {
            p.translation += vec3(0., -120., 0.) * time.delta_seconds();
        }

        for mut c in camera_query.iter_mut() {
            if input.pressed(KeyCode::KeyE) {
                c.scale += 0.6 * time.delta_seconds();
                println!("{:?}", c);
            }
            if input.pressed(KeyCode::KeyQ) {
                c.scale += -0.6 * time.delta_seconds();
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
        mycoords.0 = world_position + Vec2::new(0.5, 0.5);
    }
}

fn pixel_selected(
    mouse_position: Res<MyWorldCoords>,
    mut pixel_selected: Query<(&mut Transform, &EntityType), With<Sprite>>
) {
    for mut pixel in pixel_selected.iter_mut() {
        if pixel.1.0 == "selected".to_string() {
            let x_value: i32 = mouse_position.0.x as i32;
            let y_value: i32 = mouse_position.0.y as i32; 
            pixel.0.translation = Vec3::new(x_value as f32, y_value as f32, 1.01);
        }
    }
}

fn pixel_interaction(
    mut world_data: Query<(&mut Sprite, &Transform), With<WorldPixel>>,
    mouse_position: Res<MyWorldCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pixels_available: ResMut<PixelsTaked>
) {
    if input.pressed(MouseButton::Left) {
        let x_value = mouse_position.0.x as i64;
        let y_value = mouse_position.0.y as i64;
        let mut click_position = Vec3::new(x_value as f32, y_value as f32, 0.9);

        for pixel in world_data.iter_mut() {
            if pixel.1.translation == click_position && take_pixel(&mut click_position, &pixels_available){
               //pixel.0.color = Color::GREEN;
               spawn_machine(&mut commands, &asset_server, click_position, &mut pixels_available);
            }
        }
    }
}

fn take_pixel(position: &mut Vec3, pixels_available: &ResMut<PixelsTaked>) -> bool {
    for pixel_position in pixels_available.0.iter() {
        if pixel_position == position {
            println!("---> alredy taked: pixel {:?} - click {:?}", pixel_position, position);
            return false;
        }
    }
    true
}

fn spawn_machine(commands: &mut Commands, asset_server: &AssetServer, position: Vec3, pixels_available: &mut PixelsTaked) {
    pixels_available.0.push(position);
    println!("turtle spawned at {:?}", position);
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
        EntityType("turtle".to_string())
    ));
}