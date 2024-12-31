mod entity;
mod game;
mod res;
mod components;
mod ui;
mod utils;

use std::error::Error;
use egui_tetra2::StateWrapper;
use tetra::{ ContextBuilder};
use tetra::time::Timestep;
use crate::game::{GameState, setting};


//Context 保存框架管理的所有全局状态 窗口设置/图形/音频/硬件输入
//通过ContextBuilder构建游戏

//State用来存储游戏状态，公开了游戏循环期间调用的各种方法
fn main() -> Result<(), Box<dyn Error>>{
    //这里是传过来一格初始化GameState的函数
    
    let window_width;
    let window_height;
    {
        let mut game_setting = setting::GAME_SETTING.lock().unwrap();
        
        game_setting.window_width = 1280.0;
        game_setting.window_height = 720.0;
        window_width = game_setting.window_width as i32;
        window_height = game_setting.window_height as i32;
    }

    //建立上下文
    ContextBuilder::new("Tetra Stress Test", window_width,window_height)
        .show_mouse(true)
        .quit_on_escape(true)
        .timestep(Timestep::Fixed(60.0))
        .build()?
        .run(|ctx| Ok(StateWrapper::<Box<dyn Error>>::new(GameState::new(ctx)?)))
}









