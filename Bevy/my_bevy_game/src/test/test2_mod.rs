use std::time::Duration;
use bevy::app::{App, Startup};
use bevy::asset::{Assets, AssetServer};
use bevy::core::Name;
use bevy::input::*;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::{TextureAtlas};
use bevy::time::{Time, TimerMode};

/**
1. 坐标系
2. log宏
3. 输入系统
4. 加载图片
5. 相机
6. 帧动画模组
 */
pub struct Test2Plugin;

impl Plugin for Test2Plugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(Startup,setup_camera)
            .add_systems(Startup,(add_people,load_texture_atlas))
            //.add_systems(Startup, load_sprite)
            .add_systems(Update,(check_input,animate_sprite,camera_move))
            .register_type::<AnimationConfig>();
        

    }
}

#[derive(Component,Reflect)]
struct Person;

fn add_people(mut commands: Commands) {
    //可视化坐标包
    commands.spawn((Person,Name::new("Elaina Proctor"),
                    Transform::from_scale(Vec3::splat(3.0)),
                    Visibility::Hidden));
    //坐标包
    commands.spawn((Person,Name::new("Renzo Hume"),Transform::from_scale(Vec3::splat(3.0))));
    commands.spawn((Person,Name::new("Zana Nieves")));
    info!("完成创建")
}
///3.输入系统
fn check_input(time:Res<Time>,keys:Res<ButtonInput<KeyCode>>){
    if keys.just_pressed(KeyCode::Space) {
        info!("空格按下{:?}",time.delta())
    }

}

fn camera_move(keys:Res<ButtonInput<KeyCode>>, mut camera_query:Query<(&mut Transform,&mut OrthographicProjection),With<MyCameraMarker>>)
{
    for (mut q,mut p) in camera_query.iter_mut() {
        if keys.pressed(KeyCode::KeyW)|| keys.pressed(KeyCode::ArrowUp) {
            q.translation.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS)|| keys.pressed(KeyCode::ArrowDown) {
            q.translation.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA)|| keys.pressed(KeyCode::ArrowLeft) {
            q.translation.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD)|| keys.pressed(KeyCode::ArrowRight) {
            q.translation.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyZ) {
            p.scale += 0.01;
        }
        if keys.pressed(KeyCode::KeyX) {
            p.scale -= 0.01;
        }
    }

}

///4. 加载图片
fn load_sprite(mut commands: Commands, asset_server:Res<AssetServer>){
    let handle = asset_server.load("character_robot_sheet.png");
    commands.spawn( Sprite::from_image(handle));
}


///5. 相机模组
#[derive(Component)]
struct MyCameraMarker;
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MyCameraMarker,
    ));
}

///6.动画模组
#[derive(Component,Reflect)]
struct AnimationConfig {
    first:usize,
    last:usize,
    fps: u8,
    frame_timer: Timer
}

impl AnimationConfig {

    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first: first,
            last: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }
    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>){
    for (mut config,mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas{
                if atlas.index == config.last {
                    atlas.index = config.first;
                }else {
                    
                    atlas.index = atlas.index + 1;
                };
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}
//旋转sprite
// fn rotate_sprite(time:Res<Time>,mut query: Query<&mut Transform,With<AnimationTimer>>){
//     for mut q in query.iter_mut() {
//         q.rotation *= Quat::from_rotation_z(10.0*time.delta().as_secs_f32());
//     }
// }

//加载图集
fn load_texture_atlas(mut commands: Commands,
                      asset_server: Res<AssetServer>,
                      mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
                      camera_query:Query<&Transform,With<MyCameraMarker>>,
){
    let mut center = Vec3::default();
    for c in camera_query.iter() {
        center = c.translation;
    }
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(96,128),9,5,None,None);

    for i in 0..5000 {
        let row= &i % 100;
        let column = i / 100;
        let one = Vec3::new(&center.x - 500.0 + (20*row) as f32,&center.y - 250.0 + (20*column) as f32,0.0);
        //资源的引用
        let texture = asset_server.load("character_robot_sheet.png");
        let texture_atlas_handle = texture_atlas_layouts.add(texture_atlas_layout.clone());
        let animation_indices = AnimationConfig::new(i%44, 44, 30);

        commands.spawn((
            Sprite{
                image:texture.clone(),
                texture_atlas:Some(TextureAtlas {
                    layout: texture_atlas_handle,
                    index: animation_indices.first,
                }),
                ..default()
            },
            Transform::from_scale(Vec3::splat(0.2)).with_translation(one),
            animation_indices,
        ));
    }

}