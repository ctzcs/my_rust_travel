
pub mod setting;
pub mod mode;
pub mod save;

use tetra::State;
use std::error::Error;
use tetra::{Context, graphics, input};
use rand::distributions::{Distribution};
use crate::game::mode::IMode;
use crate::game::mode::sample_mode::SampleMode;


///游戏的状态
pub struct GameState{
    cur_mode:Box<dyn IMode>,
    mode : Vec<Box<dyn IMode>>
}
impl GameState{
    //开始游戏
    pub fn new(ctx:&mut Context)-> tetra::Result<GameState>{
        let sampleMode = SampleMode::new(ctx)?;
        Ok(GameState{
            cur_mode:Box::new(sampleMode),
            mode: Vec::new(),
        })
    }
}



///ui包装的state
impl egui_tetra2::State<Box<dyn Error>> for GameState {
    //UI更新
    fn ui(&mut self, ctx: &mut tetra::Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        self.cur_mode.ui(ctx,egui_ctx)
    }

    ///帧更新
    fn update(&mut self, ctx: &mut Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        self.cur_mode.update(ctx)
    }

    ///绘制游戏实体
    fn draw(&mut self, ctx: &mut Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        self.cur_mode.draw(ctx)
    }

    ///窗口或者输入事件
    fn event(&mut self, ctx: &mut Context, egui_ctx: &egui::Context,event: tetra::Event) -> Result<(), Box<dyn Error>> {
        self.cur_mode.event(ctx,event)
    }
}


// impl State for GameState {
//     ///更新游戏逻辑
//     fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
// 
//         //用于游戏更新
//         if input::is_key_down(ctx, Key::W) {
//             // self.player1.position.y -= self.player1.speed;
//             // for entity in self.entity_vec.iter_mut() {
//             //     entity.position.y -= entity.speed;
//             // }
//             self.camera.position.y -= CAMERA_MOVE_SPEED;
//         }
// 
//         if input::is_key_down(ctx, Key::S) {
//             // self.player1.position.y += self.player1.speed;
//             // for entity in self.entity_vec.iter_mut() {
//             //     entity.position.y += entity.speed;
//             // }
// 
//             self.camera.position.y += CAMERA_MOVE_SPEED;
//         }
// 
//         if input::is_key_down(ctx,Key::A) {
//             self.camera.position.x -= CAMERA_MOVE_SPEED;
//         }
// 
//         if input::is_key_down(ctx,Key::D) {
//             self.camera.position.x += CAMERA_MOVE_SPEED;
//         }
// 
//         if input::is_mouse_scrolled_up(ctx) {
//             self.camera.scale += CAMERA_ZOOM_SPEED;
//         }
// 
//         if input::is_mouse_scrolled_down(ctx) {
//             self.camera.scale -= CAMERA_ZOOM_SPEED;
//         }
//         
//         self.camera.update();
// 
//         let mut rng = rand::thread_rng();
//         let normal_dist = StandardNormal;
//         let mouse_pos =  self.camera.mouse_position(ctx);// input::get_mouse_position(ctx);
//         
//         for item in self.entity_vec.iter_mut() {
//             let random_value:f32 = normal_dist.sample(&mut rng);
//         
//             //if()
//             let distance = mouse_pos.distance(item.position);
//             let mut dir =  (item.position - mouse_pos).normalized();
//             if distance < DISTANCE_LIMIT {
//                 if !item.is_still_in { 
//                     //item.temp_pos = item.position;
//                 }
//                 else { item.set_still_in(true); }
//                 
//                 item.position += dir  * DISTANCE_LIMIT;
//                 
//             }else {
//                 item.set_still_in(false);
//                 item.return_to_pos();
//             }
//         
//         }
// 
//         for hero in &mut self.heros {
//             match hero {
//                 Hero::None=>{
//                     // let old_man = hero::OldMan::new("hero:old_man".to_string(),VelPos::new(Vec2::new(0.0,0.0),Vec2::new(0.0,0.0)));
//                     // self.hero = Hero::OldMan(old_man);
//                 },
//                 Hero::OldMan(oldMan)=>{
//                     oldMan.update();
//                 },
//             }
//         }
//         
// 
// 
// 
// 
//         Ok(())
//     }
// 
//     ///绘制游戏实体
//     fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
// 
//         
//         //canvas类似render texture可以用来缓存graphics的渲染结果
//         graphics::set_canvas(ctx,self.scaler.canvas());
//         // Cornflower blue, as is tradition
//         //游戏绘制
//         graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
// 
//         //绘制相机矩阵中的东西
//         graphics::set_transform_matrix(ctx,self.camera.as_matrix());
//         //在16，16的位置绘图
//         //当你传入一个 Vec2 时，它会自动转换成一个带有 position 参数集的 DrawParams 结构。
//         // 如果你想改变其他参数，比如旋转、颜色或比例，你可以使用 DrawParams::new 来构造你自己的 DrawParams
//         self.player1.texture.draw(ctx,self.player1.position);
//         self.player2.texture.draw(ctx,self.player2.position);
//         
//         for i in self.entity_vec.iter() {
//             //绘制的时候得投影到相机坐标系下
//             i.texture.draw(ctx,i.position);
//         }
//         
//         
//         //鼠标的位置也是相机坐标系下
//         self.mouse_texture.draw(ctx,self.camera.mouse_position(ctx));
//         for hero in &mut self.heros {
//             match hero {
//                 Hero::OldMan(OldMan)=>{OldMan.draw(ctx)},
//                 Hero::None=>{},
//             } 
//         }
//         
// 
// 
//         //重置矩阵，绘制固定的东西
//         graphics::reset_transform_matrix(ctx);
//         graphics::reset_canvas(ctx);
//         graphics::clear(ctx,Color::BLACK);
//         self.scaler.draw(ctx);
//         let font_data :&[u8] = res::fonts::FONT_FUSION; 
//         //let FONT = Font::vector(ctx,"resources/fonts/fusion_zh.ttf",16.0)?;
//         let font = Font::from_vector_file_data(ctx,font_data,16.0)?;
//         let mut text = graphics::text::Text::new(format!("{:.1}",get_fps(ctx)) ,font);
//         text.draw(ctx,Vec2{x:10.0,y:10.0});
//         
//         Ok(())
//     }
// 
//     ///窗口或者输入事件
//     fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
// 
//         Ok(())
//     }
// }