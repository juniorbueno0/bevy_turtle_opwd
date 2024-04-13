use perlin2d::PerlinNoise2D;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Debug)]
struct PixelPosition(Transform);

#[derive(Component, Debug)]
struct PixelColor(Color);

#[derive(Component, Debug)]
struct PixelType(String);

#[derive(Component, Debug)]
enum WorldEntityType {
    IsDeepWater(PixelType, PixelPosition, PixelColor),
    IsNormalWater(PixelType, PixelPosition, PixelColor),
    IsSand(PixelType, PixelPosition, PixelColor),
    IsGrass(PixelType, PixelPosition, PixelColor),
    IsStone(PixelType, PixelPosition, PixelColor),
    IsMineral(PixelType, PixelPosition, PixelColor)
}

#[derive(Resource, Debug)]
pub struct PixelData(Vec<WorldEntityType>);

const WORLD_X_SIZE: i32 = 400;
const WORLD_Y_SIZE: i32 = 400;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app:&mut App) {
        app.insert_resource(PixelData(Vec::new()))
            .add_systems(Startup, (generate_world_pixel, generate_world_data));
    }
}

fn generate_world_data(mut pixel_list_data: ResMut<PixelData>) {
    println!("data generated!");
    let mut rng = rand::thread_rng();
    let mut random_value: f64 = rng.gen();

    let perlin = PerlinNoise2D::new(7, 1.0, 0.6, 1.0, 2.0, (-600.0, 600.0), random_value, 101);

    let mut x: i32 = 0;
    let mut y: i32 = 0; 

    while x < WORLD_X_SIZE {
        while y < WORLD_Y_SIZE {
            let val = perlin.get_noise(x as f64, y as f64);
            println!("{:?}", val);

            random_value = rng.gen();
            if val <= -0.25 { // deep water
                let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                    PixelType("water_deep".to_string()), 
                    PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                    PixelColor(Color::rgb(0.3, 0.7, 0.9))
                );
                pixel_list_data.0.push(new_entity);
            }
            if val > -0.25 && val <= -0.5 { // normal water 
                let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                    PixelType("normal_deep".to_string()), 
                    PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                    PixelColor(Color::rgb(0.3, 0.7, 0.9))
                );
                pixel_list_data.0.push(new_entity);
               //generate_pixel(&mut commands, Color::rgb(0.3, 0.7, 0.9), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("water0".to_string()));
            }
            if val > -0.5 && val <= -0.25 { // sand
                let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                    PixelType("sand".to_string()), 
                    PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                    PixelColor(Color::rgb(0.3, 0.76, 0.9))
                );
                pixel_list_data.0.push(new_entity);
                //generate_pixel(&mut commands,Color::rgb(0.3, 0.76, 0.9), Transform::from_xyz(x as f32, y as f32, 0.9), WorldPixel("sand0".to_string()));
            }
            if val > -0.25 && val <= 0. { // grass
                let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                    PixelType("grass".to_string()), 
                    PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                    PixelColor(Color::rgb(1., 0.9, 0.5))
                );
                pixel_list_data.0.push(new_entity);
                //generate_pixel(&mut commands,Color::rgb(1., 0.9, 0.5),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("grass0".to_string()));
            }
            if val > 0. && val <= 0.5 { // mountain
                let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                    PixelType("mountain".to_string()), 
                    PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                    PixelColor(Color::rgb(0.5, 0.9, 0.4))
                );
                pixel_list_data.0.push(new_entity);
                //generate_pixel(&mut commands,Color::rgb(0.5, 0.9, 0.4),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("mountain0".to_string()));
            }
            if val > 0.5 { 
                if random_value < 0.92 {
                    let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                        PixelType("stone".to_string()), 
                        PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                        PixelColor(Color::rgb(0.63, 0.64, 0.67))
                    );
                    pixel_list_data.0.push(new_entity);
                    //generate_pixel(&mut commands,Color::rgb(0.63, 0.64, 0.67), Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("stone0".to_string()));
                } else {
                    let new_entity: WorldEntityType = WorldEntityType::IsDeepWater(
                        PixelType("mineral_gold".to_string()), 
                        PixelPosition(Transform::from_xyz(x as f32, y as f32, 0.9)), 
                        PixelColor(Color::rgb(1., 1., 0.4))
                    );
                    pixel_list_data.0.push(new_entity);
                    //generate_pixel(&mut commands,Color::rgb(1., 1., 0.4),Transform::from_xyz(x as f32, y as f32, 0.9),WorldPixel("gold".to_string()));
                }
            }

            y += 1;
        }   
        y = 0;
        x += 1;
    } 
}

fn generate_world_pixel(mut commands: Commands, pixel_data: Res<PixelData>) {
    println!("pixels drawed");
    for data in pixel_data.0.iter() {

        match data {
            WorldEntityType::IsDeepWater(pixel_type, pixel_position, pixel_color) => {
                commands.spawn(SpriteBundle {
                    sprite: Sprite { color: pixel_color.0, ..default() },
                    transform: pixel_position.0,
                    ..default()
                }).insert(PixelType(pixel_type.0.to_string()));
            },
            WorldEntityType::IsNormalWater(pixel_type, pixel_position, pixel_color) => {
                commands.spawn(SpriteBundle {
                    sprite: Sprite { color: pixel_color.0, ..default() },
                    transform: pixel_position.0,
                    ..default()
                }).insert(PixelType(pixel_type.0.to_string()));
            },
            WorldEntityType::IsSand(pixel_type, pixel_position, pixel_color) => {
                commands.spawn(SpriteBundle {
                    sprite: Sprite { color: pixel_color.0, ..default() },
                    transform: pixel_position.0,
                    ..default()
                }).insert(PixelType(pixel_type.0.to_string()));
            },
            WorldEntityType::IsGrass(pixel_type, pixel_position, pixel_color) => {
                commands.spawn(SpriteBundle {
                    sprite: Sprite { color: pixel_color.0, ..default() },
                    transform: pixel_position.0,
                    ..default()
                }).insert(PixelType(pixel_type.0.to_string()));
            },
            WorldEntityType::IsStone(pixel_type, pixel_position, pixel_color) => {
                commands.spawn(SpriteBundle {
                    sprite: Sprite { color: pixel_color.0, ..default() },
                    transform: pixel_position.0,
                    ..default()
                }).insert(PixelType(pixel_type.0.to_string()));
            },
            WorldEntityType::IsMineral(pixel_type, pixel_position, pixel_color) => {
                commands.spawn(SpriteBundle {
                    sprite: Sprite { color: pixel_color.0, ..default() },
                    transform: pixel_position.0,
                    ..default()
                }).insert(PixelType(pixel_type.0.to_string()));
            }
        }
    }
}