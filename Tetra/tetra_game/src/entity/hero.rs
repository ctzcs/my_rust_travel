use std::time::Duration;
use rand::Rng;
use tetra::{Context, graphics};
use tetra::graphics::animation::Animation;
use tetra::graphics::{animation, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use crate::components::{position::VelPos, view::View};
use crate::entity::{BaseEntity, IEntity};
use crate::game::mode::sample_mode::{SampleMode, SampleModeModel};
use crate::game::setting::{TIME_SETTING, TimeSetting};
use crate::utils;

#[derive(Debug)]
pub struct SampleCha {
    base:BaseEntity,
    name:String,
    vel_pos:VelPos,
    view: View,
    start_pos:Vec2<f32>,
    time:f32,
}

impl SampleCha {
    pub fn new(id:u32,name:String, texture: Texture, vel_pos: VelPos) -> SampleCha { 
        let index = rand::thread_rng().gen_range(0..10);
        let start_pos = vel_pos.get_position().clone();
       let mut cha = SampleCha {
            base: BaseEntity { id },
            name: name,
            vel_pos: vel_pos,
            view: View::new(
                Animation::new(
                    texture,
                    Rectangle::row(0.0, 0.0, 32.0, 32.0).take(10).collect(),
                    Duration::from_secs_f64(0.1f64))),
           start_pos, 
           time:0.0
       };
        cha.view.anim.set_current_frame_index(index);
        cha
    }
}

impl IEntity for SampleCha {
    fn get_id(&self) -> &u32 {
        &self.base.id
    }

    fn update(&mut self, ctx: &mut Context,sample_mode_model:&mut SampleModeModel) {
        let distance = Vec2::distance(self.vel_pos.get_position().clone(),Vec2::zero());
        self.time += TIME_SETTING.lock().unwrap().fixed_delta_time;
        let pos =self.start_pos + utils::get_position(Vec2::zero(),distance/10.0,self.time,1.0);
        self.vel_pos.set_position(pos);
        // self.vel_pos.set_position(
        //     self.vel_pos.get_position() + Vec2::new(0.01, 0.01));
        
        //根据model更新自己的状态
    }

    fn draw(&mut self, ctx: &mut Context) {
        //画出当前状态
        
        //如果要让动画播放，必须先设置advance
        self.view.anim.advance(ctx);
        self.view.anim.draw(ctx,DrawParams::new()
            .position(*self.vel_pos.get_position())
            .origin(Vec2::new(32.0, 32.0))
            .scale(Vec2::new(1.0, 1.0)),);
    }
    
}

impl PartialEq for SampleCha {
    fn eq(&self, other: &Self) -> bool {
        if self.base == other.base { 
            true
        }else {
            false
        }
        
    }
}
