use std::str::CharIndices;
use std::sync::Mutex;
use std::time::Duration;
use rand::Rng;
use tetra::{Context, graphics};
use tetra::graphics::animation::Animation;
use tetra::graphics::{animation, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::window::set_key_repeat_enabled;
use crate::components::{position::VelPos, view::View};
use crate::entity::{BaseEntity, IEntity};
use crate::game::GameState;
use crate::res::Assets;

pub enum Hero{
    SampleCha(SampleCha),
    None
}

#[derive(Debug)]
pub struct SampleCha {
    base:BaseEntity,
    name:String,
    vel_pos:VelPos,
    view: View,
}

impl SampleCha {
    pub fn new(state:&mut GameState,name:String, texture: Texture, vel_pos: VelPos) -> SampleCha { 
        let index = rand::thread_rng().gen_range(0..10);
       let mut cha = SampleCha {
            base: BaseEntity { id: state.id_allocator.pop_id() },
            name: name,
            vel_pos: vel_pos,
            view: View::new(
                Animation::new(
                    texture,
                    Rectangle::row(0.0, 0.0, 32.0, 32.0).take(10).collect(),
                    Duration::from_secs_f64(0.1f64))),
        };
        cha.view.anim.set_current_frame_index(index);
        cha
    }
}

impl IEntity for SampleCha {
    fn get_id(&self) -> &u32 {
        &self.base.id
    }

    fn update(&mut self, ctx: &mut Context) {
        self.vel_pos.set_position(
            self.vel_pos.get_position() + Vec2::new(0.01, 0.01));
    }

    fn draw(&mut self, ctx: &mut Context) {
        //如果要让动画播放，必须先设置advance
        self.view.anim.advance(ctx);
        self.view.anim.draw(ctx,DrawParams::new()
            .position(*self.vel_pos.get_position())
            .origin(Vec2::new(32.0, 32.0))
            .scale(Vec2::new(1.0, 1.0)),);
    }
}
