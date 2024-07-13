use std::rc::Rc;
use std::sync::Mutex;
use std::time::Duration;
use tetra::{Context, graphics};
use tetra::graphics::animation::Animation;
use tetra::graphics::{animation, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use crate::entity_component::{position::VelPos,view::AnimView};
use crate::res;

pub enum Hero{
    OldMan(OldMan),
    None
}

#[derive(Debug)]
pub struct OldMan{
    name:String,
    velpos:VelPos,
    view:AnimView,
}

impl OldMan{
    pub fn new(name:String, texture: Texture, vel_pos: VelPos) -> OldMan{
        OldMan{
            name:name,
            velpos:vel_pos,
            view:AnimView::new(
                Animation::new(
                texture,
                Rectangle::row(0.0,0.0,32.0,32.0).take(10).collect(), 
                Duration::from_secs_f64(0.1))),
        }
    }

    pub fn update(&mut self){
        self.velpos.set_position(
            self.velpos.get_position() + Vec2::new(0.01,0.01));
        //println!("{:?}:{:?}", self.name, self.velpos);
    }
    
    pub  fn draw(&mut self,ctx:&mut Context){
        //如果要让动画播放，必须先设置advance
        self.view.anim.advance(ctx);
        
        self.view.anim.draw(ctx,DrawParams::new()
            .position(*self.velpos.get_position())
            .origin(Vec2::new(32.0, 32.0))
            .scale(Vec2::new(1.0, 1.0)),);
    }
}