use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use rand::{random, Rng, thread_rng};
use tetra::{Context, graphics, input};
use tetra::graphics::text::Font;
use tetra::graphics::{Camera, Color, Texture};
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::time::get_fps;
use crate::components::position::VelPos;
use crate::entity::character::{sample_cha::SampleCha};
use crate::entity::character::sample_cha2::SampleCha2;
use crate::entity::id::IdAllocator;
use crate::entity::manager::EntityManager;
use crate::game::mode::{IMode, IModelData, Model};
use crate::game::setting::GAME_SETTING;
use crate::res;
use crate::utils::screen_to_world;


const LINE_COUNT:u32 = 50;
const ROW_COUNT:u32 = 100;
const SPACE:Vec2<f32> = Vec2::new(32.0,32.0);
const CAMERA_MOVE_SPEED:f32 = 30.0;
const CAMERA_ZOOM_SPEED:f32 = 0.1;
const DISTANCE_LIMIT:f32 = 50.0;
const PANEL_SPEED:f32= 2.0;

pub struct SampleMode {
    pub camera:Camera,
    pub scaler: ScreenScaler,
    pub assets: Assets,
    pub model:Model<SampleModel>,
    
}



impl SampleMode{
    //开始游戏
    pub fn new(ctx:&mut Context)-> tetra::Result<SampleMode>{
        let total = LINE_COUNT * ROW_COUNT;
        let game_setting = GAME_SETTING.lock().unwrap();
        let window_width = game_setting.window_width;
        let window_height = game_setting.window_height;
        //资源加载,将其打包到可执行文件中
        //解码Texture
        //这里只加载了一份Texture到内存中，如果每次都用这个创建，就会每次都new，如果新的东西需要使用这个贴图，只需要clone()就好
        let mouse_texture = Rc::new(Texture::from_encoded(ctx,res::textures::MOUSE)?);
        let mut mode = SampleMode{
            scaler:ScreenScaler::with_window_size(ctx,window_width as i32,window_height as i32,ScalingMode::ShowAllPixelPerfect)?,
            camera:Camera::new(window_width,window_height),
            // player1: EntityBase::new(Rc::clone(&texture), player1_position, paddle_speed),
            // player2: EntityBase::new(Rc::clone(&texture), player2_position, paddle_speed),
            // entity_vec:many_entity,
            
            assets:Assets{mouse_texture},
            model:Model::<SampleModel>{
                data:Rc::new(RefCell::new(SampleModel{
                    id_allocator:IdAllocator::new(0,Vec::new()),
                    entity_manager: EntityManager::new(HashMap::new()),
                }))
                
            }
        };
        
        create_hero(&mut mode,ctx);

        Ok(mode)
    }
    
}

///ui包装的state
impl IMode for SampleMode {
    //UI更新

    fn ui(&mut self, ctx: &mut tetra::Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        egui::Window::new("ui1").show(egui_ctx,|ui|{
            ui.label("hello")
        });

        egui::Window::new("ui2").show(egui_ctx,|ui|{
            ui.label("hello1")
        });
        Ok(())
    }
    ///帧更新
    fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {

        //用户输入
        if input::is_key_down(ctx, Key::W) {
            // self.player1.position.y -= self.player1.speed;
            // for entity in self.entity_vec.iter_mut() {
            //     entity.position.y -= entity.speed;
            // }
            self.camera.position.y -= CAMERA_MOVE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            // self.player1.position.y += self.player1.speed;
            // for entity in self.entity_vec.iter_mut() {
            //     entity.position.y += entity.speed;
            // }

            self.camera.position.y += CAMERA_MOVE_SPEED;
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

        //相机更新
        self.camera.update();
        let model = self.model.clone();
        model.get_mut_data().entity_manager.update(ctx, self.model.clone());
        //model.data.borrow_mut().entity_manager.update(ctx, self.model.clone() /*&mut *self.model.data.borrow_mut()*/);
        //self.model.entity_manager.update(ctx, self.model.clone() /*&mut *self.model.data.borrow_mut()*/);
        
        Ok(())
    }

    ///绘制游戏实体
    fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {


        //canvas类似render texture可以用来缓存graphics的渲染结果
        graphics::set_canvas(ctx,self.scaler.canvas());
        // Cornflower blue, as is tradition
        //游戏绘制
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        //绘制相机矩阵中的东西,应用相机变换，直接绘制相机中的世界坐标系的东西
        graphics::set_transform_matrix(ctx,self.camera.as_matrix());
        //在16，16的位置绘图
        //当你传入一个 Vec2 时，它会自动转换成一个带有 position 参数集的 DrawParams 结构。
        // 如果你想改变其他参数，比如旋转、颜色或比例，你可以使用 DrawParams::new 来构造你自己的 DrawParams
        
        // for i in self.entity_vec.iter() {
        //     //绘制的时候得投影到相机坐标系下
        //     i.texture.draw(ctx,i.position);
        // }

        
        // if let Some(heros)  = self.model.get_mut_data().entity_manager.get_entities::<SampleCha>(){
        //     for hero in heros.iter_mut() {
        //         hero.draw(ctx);
        //     }
        // }
        
        self.model.get_mut_data().entity_manager.draw(ctx);
        

        //鼠标的位置也是相机坐标系下
        let mouse_world_position = screen_to_world(&self.camera, input::get_mouse_position(ctx));
        let mouse_camera_position = self.camera.mouse_position(ctx);
        self.assets.mouse_texture.draw(ctx,
                                       mouse_camera_position);

        println!("鼠标的屏幕坐标系{0}",input::get_mouse_position(ctx));
        println!("鼠标世界坐标系位置{0}", mouse_world_position);
        println!("鼠标相机坐标系位置{0}",mouse_camera_position);

        

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
    fn event(&mut self, ctx: &mut Context,event: tetra::Event) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}



fn create_hero(mode:&mut SampleMode,ctx:&mut Context){
    
    let circle_texture = Texture::from_encoded(ctx, res::textures::CIRCLE_TEXTURE).unwrap();
    let rocket_texture = Texture::from_encoded(ctx,res::textures::ROCKET_TEXTURE).unwrap();
    // let mut tex = Vec::<Texture>::new();
    // tex.push(circle_texture);
    // tex.push(rocket_texture);
    
    let textures = [circle_texture,rocket_texture];
    let mut random = rand::thread_rng();
    
    //每次都应该画所有同样的东西
    for i in 0.. ROW_COUNT{
        for j in 0..LINE_COUNT {
            let index= random.gen_range(0..2);
            let position = Vec2::new(j as f32 * SPACE.x,i as f32 * SPACE.y);
            let hero = SampleCha::new(mode.model.data.borrow_mut().id_allocator.pop_id(), "Old_man".to_string(),textures[0].clone(), VelPos::new(position, Vec2::new(0.0, 0.0)));
            let _ = mode.model.get_mut_data().entity_manager.add::<SampleCha>(Box::new(hero));
        }
    }


    for i in 0..ROW_COUNT {
        for j in 0..LINE_COUNT {
            let index= random.gen_range(0..2);
            let position =  Vec2::new((j) as f32 * SPACE.x,(i+ROW_COUNT) as f32 * SPACE.y);
            let hero = SampleCha2::new(mode.model.data.borrow_mut().id_allocator.pop_id(), "Man2".to_string(), textures[1].clone(), VelPos::new(position, Vec2::new(0.0, 0.0)));
            let _ = mode.model.get_mut_data().entity_manager.add::<SampleCha2>(Box::new(hero));
        }
    }
}

pub struct Assets{
    pub mouse_texture:Rc<Texture>,
}

pub struct SampleModel{
    pub id_allocator:IdAllocator,
    pub entity_manager: EntityManager<SampleModel>
}

impl IModelData for SampleModel {
    
}