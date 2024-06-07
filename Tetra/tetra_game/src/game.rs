pub mod setting;

use tetra::{Context, Event, graphics, input, State, TetraError};
use tetra::graphics::{Camera, Color, Texture};
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::Font;
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::time::get_fps;
use rand::distributions::{Distribution};
use rand_distr::StandardNormal;
use crate::entity::EntityBase;
use crate::game::setting::{GAME_SETTING};


const PANEL_COUNT:i32 = 500000;
const CAMERA_MOVE_SPEED:f32 = 10.0;
const CAMERA_ZOOM_SPEED:f32 = 0.1;
///游戏的状态
pub struct GameState{
    pub camera:Camera,
    pub scaler: ScreenScaler,
    pub player1: EntityBase,
    pub player2: EntityBase,
    pub entity_vec:Vec<EntityBase>
}
impl GameState{
    //开始游戏
    pub fn new(ctx:&mut Context)-> tetra::Result<GameState>{

        let line_count = PANEL_COUNT/100;
        let game_setting = GAME_SETTING.lock().unwrap();
        let window_width = game_setting.window_width;
        let window_height = game_setting.window_height;
        let player1_texture = Texture::new(ctx,"./resources/player1.png")?;
        let player1_position = Vec2::new(16.0,( window_width - player1_texture.height() as f32)/2.0);
        let player2_position = Vec2::new(window_width - player1_texture.width() as f32 - 16.0,(window_height - player1_texture.height() as f32)/2.0);
        let paddle_speed = 8.0;
        let mut many_entity = Vec::<EntityBase>::new();
        for i in 0.. PANEL_COUNT {
            let position = Vec2::new( (i%line_count) as f32*10.0,(i / line_count) as f32 * player1_texture.height() as f32);
            let _ = &mut many_entity.push(EntityBase::new(player1_texture.clone(), position, paddle_speed),);
        }
        Ok(GameState{
            scaler:ScreenScaler::with_window_size(ctx,window_width as i32,window_height as i32,ScalingMode::ShowAllPixelPerfect)?,
            camera:Camera::new(window_width,window_height),
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

        if input::is_key_down(ctx,Key::A) {
            self.camera.position.x -= CAMERA_MOVE_SPEED;
        }

        if input::is_key_down(ctx,Key::D) {
            self.camera.position.x += CAMERA_MOVE_SPEED;
        }

        if input::is_mouse_scrolled_up(ctx) {
            self.camera.scale += CAMERA_ZOOM_SPEED;
        }

        if input::is_mouse_scrolled_down(ctx) {
            self.camera.scale -= CAMERA_ZOOM_SPEED;
        }
        
        self.camera.update();

        let mut rng = rand::thread_rng();
        let normal_dist = StandardNormal;
        for item in self.entity_vec.iter_mut() {
            let random_value:f32 = normal_dist.sample(&mut rng);
            item.position += random_value;
        }



        Ok(())
    }

    ///绘制游戏实体
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        //canvas类似render texture可以用来缓存graphics的渲染结果
        graphics::set_canvas(ctx,self.scaler.canvas());
        // Cornflower blue, as is tradition
        //游戏绘制
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        //绘制相机矩阵中的东西
        graphics::set_transform_matrix(ctx,self.camera.as_matrix());
        //在16，16的位置绘图
        //当你传入一个 Vec2 时，它会自动转换成一个带有 position 参数集的 DrawParams 结构。
        // 如果你想改变其他参数，比如旋转、颜色或比例，你可以使用 DrawParams::new 来构造你自己的 DrawParams
        self.player1.texture.draw(ctx,self.player1.position);
        self.player2.texture.draw(ctx,self.player2.position);
        for i in self.entity_vec.iter() {
            i.texture.draw(ctx,i.position);
        }

        //重置矩阵，绘制固定的东西
        graphics::reset_transform_matrix(ctx);
        graphics::reset_canvas(ctx);
        graphics::clear(ctx,Color::BLACK);
        self.scaler.draw(ctx);
        let font = Font::vector(ctx,"resources/fonts/fusion_zh.ttf",16.0);
        let mut text = graphics::text::Text::new(format!("{:.1}",get_fps(ctx)) ,font.unwrap());
        text.draw(ctx,Vec2{x:10.0,y:10.0});
        Ok(())
    }

    ///窗口或者输入事件
    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {

        Ok(())
    }
}
