mod entity;
mod game;
use tetra::{ ContextBuilder};
use crate::game::{GameState, setting};


//Context 保存框架管理的所有全局状态 窗口设置/图形/音频/硬件输入
//通过ContextBuilder构建游戏

//State用来存储游戏状态，公开了游戏循环期间调用的各种方法
fn main() -> tetra::Result{
    //这里是传过来一格初始化GameState的函数
    let game_state = GameState::new;
    let window_width;
    let window_height;
    {
        let game_setting = setting::GAME_SETTING.lock().unwrap();
        window_width = game_setting.window_width as i32;
        window_height = game_setting.window_height as i32;
    }

    //建立上下文
    ContextBuilder::new("Hello, world!", window_width,window_height)
        .quit_on_escape(true)
        .build()?
        .run(game_state)

}









