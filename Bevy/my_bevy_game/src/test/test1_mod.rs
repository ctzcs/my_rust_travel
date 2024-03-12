use bevy::app::{App, Plugin, PostStartup, Startup};
use bevy::prelude::*;
use bevy_inspector_egui;
/**
该模块学习了：
1. 实体
2. 组件
3. 系统和事件处理
4. 事件
5. 插件
6. editor_gui注册
 */

//插件包
pub struct Test1Plugin;
impl Plugin for Test1Plugin {
    fn build(&self, app: &mut App) {
        /*将需要运行的系统添加到bevy中，元组最多支持16个元素，但可以无限嵌套
            Startup->Update */
        app
            .add_systems(Startup,add_people)
            .add_systems(PostStartup,(greet_people,print_position_system))
            .add_event::<LevelUpEvent>()
            //注册是为了在inspector中可以显示
            .register_type::<MyPosition>()
            .register_type::<PlayerXp>()
            .register_type::<Person>();
    }
}

//1. Entity
//struct Entity(u64);
#[derive(Component,Reflect)]
struct Person;
#[derive(Component,Reflect)]
struct MyPosition {
    x:f32,
    y:f32
}
#[derive(Component,Reflect)]
struct PlayerXp(u32);




//2. Systems
///添加人物的System
fn add_people(mut commands: Commands) {
    commands.spawn((Person, MyPosition {x:0.1,y:0.1}, Name::new("Elaina Proctor")));
    commands.spawn((Person, MyPosition {x:0.2,y:0.2}, Name::new("Renzo Hume")));
    commands.spawn((Person, MyPosition {x:0.3,y:0.3}, Name::new("Zayna Nieves")));
}

///删除人物的方法
fn delete_people(entity: Entity,mut commands:Commands)
{
    commands.entity(entity).despawn();
}

///查询人物
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name);
    }
}

///打印人物位置的系统
fn print_position_system(query: Query<&MyPosition,With<Name>>) {
    for transform in &query {
        println!("position: {},{:?}", transform.x,transform.y);
    }
}

///人物升级事件处理系统
fn player_level_up(
    mut event_level_up: EventWriter<LevelUpEvent>,
    query: Query<(Entity, &PlayerXp)>)
{
    for (entity, xp) in query.iter() {
        if xp.0 > 1000 {
            //升级的时候，事件激活
            event_level_up.send(LevelUpEvent(entity));
        }
    }
}
///升级事件处理系统
fn debug_levelups(mut event_level_up: EventReader<LevelUpEvent>) {
    //接受事件
    for ev in event_level_up.read() {
        eprintln!("Entity {:?} leveled up!", ev.0);
    }
}

//4. 升级事件
#[derive(Event)]
struct LevelUpEvent(Entity);






