
pub mod setting;

use tetra::State;
use std::error::Error;
use std::rc::Rc;
use tetra::{Context, Event, graphics, input};
use tetra::graphics::{Camera, Color, Texture};
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::Font;
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::time::{get_fps};
use rand::distributions::{Distribution};
use rand_distr::StandardNormal;
use crate::entity::{hero,EntityBase};
use crate::entity::hero::{Hero, OldMan};
use crate::components::position::VelPos;
use crate::game::setting::{GAME_SETTING};
use crate::res;


const PANEL_COUNT:i32 = 1000;
const CAMERA_MOVE_SPEED:f32 = 30.0;
const CAMERA_ZOOM_SPEED:f32 = 0.1;
const DISTANCE_LIMIT:f32 = 50.0;
const PANEL_SPEED:f32= 2.0;

///游戏的状态
pub struct GameState{
    pub camera:Camera,
    pub scaler: ScreenScaler,
    pub player1: EntityBase,
    pub player2: EntityBase,
    pub heros:Vec<Hero>,
    pub entity_vec:Vec<EntityBase>,
    pub mouse_texture:Rc<Texture>
}
impl GameState{
    //开始游戏
    pub fn new(ctx:&mut Context)-> tetra::Result<GameState>{
        let line_count = PANEL_COUNT/100;
        let game_setting = GAME_SETTING.lock().unwrap();
        let window_width = game_setting.window_width;
        let window_height = game_setting.window_height;
        //资源加载,将其打包到可执行文件中
        let texture_data:&[u8] = res::images::PANEL;
        //解码
        let panel_texture = Texture::from_encoded(ctx, texture_data)?;
        let texture = Rc::new(panel_texture);

        //这里只加载了一份Texture到内存中，如果每次都用这个创建，就会每次都new，如果新的东西需要使用这个贴图，只需要clone()就好
        let mouse_texture = Rc::new(Texture::from_encoded(ctx,res::images::MOUSE)?);
        let oldMan_texture = Texture::from_encoded(ctx,res::images::CIRCLE_ANIM).unwrap();

        //let player1_texture = Texture::new(ctx,"./resources/player1.png")?;
        let player1_position = Vec2::new(16.0,( window_width - texture.height() as f32)/2.0);
        let player2_position = Vec2::new(window_width - texture.width() as f32 - 16.0, (window_height - texture.height() as f32)/2.0);
        let paddle_speed = PANEL_SPEED;
        let mut many_entity = Vec::<EntityBase>::new();
        let mut many_hero = Vec::<Hero>::new();
        for i in 0.. PANEL_COUNT {
            let position = Vec2::new( (i%line_count) as f32*20.0,(i / line_count) as f32 * texture.height() as f32);
            let _ = &mut many_entity.push(EntityBase::new(Rc::clone(&texture), position, paddle_speed),);
            //通过克隆就不需要进行texture解码
            let _ = &mut many_hero.push(Hero::OldMan(OldMan::new("Old_man".to_string(),oldMan_texture.clone(), VelPos::new(position, Vec2::new(0.0, 0.0)))));
        }
        Ok(GameState{
            scaler:ScreenScaler::with_window_size(ctx,window_width as i32,window_height as i32,ScalingMode::ShowAllPixelPerfect)?,
            camera:Camera::new(window_width,window_height),
            player1: EntityBase::new(Rc::clone(&texture), player1_position, paddle_speed),
            player2: EntityBase::new(Rc::clone(&texture), player2_position, paddle_speed),
            entity_vec:many_entity,
            heros:many_hero,
            mouse_texture:mouse_texture,
        })
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

///ui包装的state
impl egui_tetra2::State<Box<dyn Error>> for GameState {
    fn ui(&mut self, ctx: &mut tetra::Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        egui::Window::new("ui1").show(egui_ctx,|ui|{
            ui.label("hello")
        });

        egui::Window::new("ui2").show(egui_ctx,|ui|{
            ui.label("hello1")
        });
        Ok(())
    }


    fn update(&mut self, ctx: &mut Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {

        //用于游戏更新
        if input::is_key_down(ctx, Key::W) {
            // self.player1.position.y -= self.player1.speed;
            // for entity in self.entity_vec.iter_mut() {
            //     entity.position.y -= entity.speed;
            // }
            self.camera.position.y -= crate::game::CAMERA_MOVE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            // self.player1.position.y += self.player1.speed;
            // for entity in self.entity_vec.iter_mut() {
            //     entity.position.y += entity.speed;
            // }

            self.camera.position.y += crate::game::CAMERA_MOVE_SPEED;
        }

        if input::is_key_down(ctx,Key::A) {
            self.camera.position.x -= crate::game::CAMERA_MOVE_SPEED;
        }

        if input::is_key_down(ctx,Key::D) {
            self.camera.position.x += crate::game::CAMERA_MOVE_SPEED;
        }

        if input::is_mouse_scrolled_up(ctx) {
            self.camera.scale += crate::game::CAMERA_ZOOM_SPEED;
        }

        if input::is_mouse_scrolled_down(ctx) {
            self.camera.scale -= crate::game::CAMERA_ZOOM_SPEED;
        }

        self.camera.update();

        let mut rng = rand::thread_rng();
        let normal_dist = StandardNormal;
        let mouse_pos =  self.camera.mouse_position(ctx);// input::get_mouse_position(ctx);

        for item in self.entity_vec.iter_mut() {
            let random_value:f32 = normal_dist.sample(&mut rng);

            //if()
            let distance = mouse_pos.distance(item.position);
            let mut dir =  (item.position - mouse_pos).normalized();
            if distance < DISTANCE_LIMIT {
                if !item.is_still_in {
                    //item.temp_pos = item.position;
                }
                else { item.set_still_in(true); }

                item.position += dir  * DISTANCE_LIMIT;

            }else {
                item.set_still_in(false);
                item.return_to_pos();
            }

        }

        for hero in &mut self.heros {
            match hero {
                Hero::None=>{
                    // let old_man = hero::OldMan::new("hero:old_man".to_string(),VelPos::new(Vec2::new(0.0,0.0),Vec2::new(0.0,0.0)));
                    // self.hero = Hero::OldMan(old_man);
                },
                Hero::OldMan(oldMan)=>{
                    oldMan.update();
                },
            }
        }





        Ok(())
    }

    ///绘制游戏实体
    fn draw(&mut self, ctx: &mut Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {


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
            //绘制的时候得投影到相机坐标系下
            i.texture.draw(ctx,i.position);
        }


        //鼠标的位置也是相机坐标系下
        self.mouse_texture.draw(ctx,self.camera.mouse_position(ctx));
        for hero in &mut self.heros {
            match hero {
                Hero::OldMan(OldMan)=>{OldMan.draw(ctx)},
                Hero::None=>{},
            }
        }



        //重置矩阵，绘制固定的东西
        graphics::reset_transform_matrix(ctx);
        graphics::reset_canvas(ctx);
        graphics::clear(ctx,Color::BLACK);
        self.scaler.draw(ctx);
        let font_data :&[u8] = res::fonts::FONT_FUSION;
        //let FONT = Font::vector(ctx,"resources/fonts/fusion_zh.ttf",16.0)?;
        let font = Font::from_vector_file_data(ctx,font_data,16.0)?;
        let mut text = graphics::text::Text::new(format!("{:.1}",get_fps(ctx)) ,font);
        text.draw(ctx,Vec2{x:10.0,y:10.0});

        Ok(())
    }

    ///窗口或者输入事件
    fn event(&mut self, ctx: &mut Context, egui_ctx: &egui::Context,event: tetra::Event) -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}