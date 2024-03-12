mod test;
mod universal;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        //bevy默认的插件
        //inspector窗口
        .add_plugins(test::test2_mod::Test2Plugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}




