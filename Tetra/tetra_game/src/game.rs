pub mod setting;

use tetra::{Context, Event, graphics, input, State, TetraError};
use tetra::graphics::{Color, Texture};
use tetra::input::Key;
use tetra::math::Vec2;
use crate::entity::EntityBase;
use crate::game::setting::{GAME_SETTING, GameSetting};


const PANEL_COUNT:i32 = 100000;
///游戏的状态
pub struct GameState{
    pub player1: EntityBase,
    pub player2: EntityBase,
    pub entity_vec:Vec<EntityBase>
}
impl GameState{
    //开始游戏
    pub fn new(ctx:&mut Context)-> tetra::Result<GameState>{

        let game_setting = GAME_SETTING.lock().unwrap();
        let window_width = game_setting.window_width;
        let window_height = game_setting.window_height;
        let player1_texture = Texture::new(ctx,"./resources/player1.png")?;
        let player1_position = Vec2::new(16.0,( window_width - player1_texture.height() as f32)/2.0);
        let player2_position = Vec2::new(window_width - player1_texture.width() as f32 - 16.0,(window_height - player1_texture.height() as f32)/2.0);
        let paddle_speed = 8.0;
        let mut many_entity = Vec::<EntityBase>::new();
        for i in 0.. PANEL_COUNT {
            let position = Vec2::new(16.0+ i as f32 * 0.01,(window_height - player1_texture.height() as f32)/2.0);
            let _ = &mut many_entity.push(EntityBase::new(player1_texture.clone(), position, paddle_speed),);
        }
        Ok(GameState{
            player1: EntityBase::new(player1_texture.clone(), player1_position, paddle_speed),
            player2: EntityBase::new(player1_texture.clone(), player2_position, paddle_speed),
            entity_vec:many_entity,
        })
    }
}

impl State for GameState {
    ///更新游戏逻辑
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        //用于游戏更新
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= self.player1.speed;
            for entity in self.entity_vec.iter_mut() {
                entity.position.y -= entity.speed;
            }
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += self.player1.speed;
            for entity in self.entity_vec.iter_mut() {
                entity.position.y += entity.speed;
            }
        }
        Ok(())
    }

    ///绘制游戏实体
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        //游戏绘制
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        //在16，16的位置绘图
        //当你传入一个 Vec2 时，它会自动转换成一个带有 position 参数集的 DrawParams 结构。
        // 如果你想改变其他参数，比如旋转、颜色或比例，你可以使用 DrawParams::new 来构造你自己的 DrawParams
        self.player1.texture.draw(ctx,self.player1.position);
        self.player2.texture.draw(ctx,self.player2.position);
        for i in self.entity_vec.iter() {
            i.texture.draw(ctx,i.position);
        }

        Ok(())
    }

    ///窗口或者输入事件
    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {

        Ok(())
    }
}
