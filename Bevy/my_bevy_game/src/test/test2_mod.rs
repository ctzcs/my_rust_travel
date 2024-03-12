use bevy::app::{App, Startup};
use bevy::asset::{Assets, AssetServer};
use bevy::core::Name;
use bevy::input::*;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas};
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
            .register_type::<AnimationIndices>()
            .register_type::<AnimationTimer>();

    }
}

#[derive(Component,Reflect)]
struct Person;

fn add_people(mut commands: Commands) {
    //可视化坐标包
    commands.spawn((Person,Name::new("Elaina Proctor"),SpatialBundle{
        transform:Transform::from_scale(Vec3::splat(3.0)),
        visibility:Visibility::Hidden,
        ..Default::default()
    }));
    //坐标包
    commands.spawn((Person,Name::new("Renzo Hume"),TransformBundle{
        local:Transform::from_scale(Vec3::splat(3.0)),
        ..Default::default()
    }));
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
    commands.spawn(SpriteBundle{
        sprite: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
        texture: handle,
        visibility: Default::default(),
        inherited_visibility: Default::default(),
        view_visibility: Default::default(),
    });
}


///5. 相机模组
#[derive(Component)]
struct MyCameraMarker;
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

///6.动画模组
#[derive(Component,Reflect)]
struct AnimationIndices{
    first:usize,
    last:usize,
}
#[derive(Component,Reflect, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(time:Res<Time>,
mut query:Query<(&AnimationIndices,&mut AnimationTimer,&mut TextureAtlas)>){
    for (indices,mut timer,mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            }else {
                sprite.index +1
            };
        }
    }
}
//旋转sprite
fn rotate_sprite(time:Res<Time>,mut query: Query<&mut Transform,With<AnimationTimer>>){
    for mut q in query.iter_mut() {
        q.rotation *= Quat::from_rotation_z(10.0*time.delta().as_secs_f32());
    }
}

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
    let texture_atlas_layout = TextureAtlasLayout::from_grid(Vec2::new(96.0,128.0),9,5,None,None);

    for i in 0..5000 {
        let row= &i % 100;
        let column = i / 100;
        let one = Vec3::new(&center.x - 500.0 + (20*row) as f32,&center.y - 250.0 + (20*column) as f32,0.0);
        //资源的引用
        let texture = asset_server.load("character_robot_sheet.png");
        let texture_atlas_handle = texture_atlas_layouts.add(texture_atlas_layout.clone());
        let animation_indices = AnimationIndices
        {
            first:i%44,
            last:44,
        };

        commands.spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_handle,
                    index: animation_indices.first,
                },
                transform: Transform::from_scale(Vec3::splat(0.2)).with_translation(one),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }

}