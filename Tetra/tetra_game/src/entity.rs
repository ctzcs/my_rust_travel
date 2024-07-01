pub mod hero;

use std::rc::Rc;
use tetra::graphics::Texture;
use tetra::math::Vec2;

pub struct EntityBase {
    pub texture:Rc<Texture>,
    pub position:Vec2<f32>,
    pub start_pos:Vec2<f32>,
    pub speed:f32,
    pub dir:Vec2<f32>,
    //pub temp_pos:Vec2<f32>,
    pub is_still_in:bool,
}
impl EntityBase {
    pub fn new(texture: Rc<Texture>,position:Vec2<f32>,speed:f32)-> EntityBase {
        //let temp_pos = position;
        let start_pos = position;
        let dir:Vec2<f32> = Vec2::new(0.0,0.0);
        EntityBase {texture,position,start_pos,speed,dir,is_still_in:false}
    }
    pub fn return_to_pos(&mut self){
        let distance = self.position.distance(self.start_pos);
        if distance > 0.1 {
            self.dir = (self.start_pos - self.position).normalized();
            self.position += self.dir * self.speed;
        }
    }
    
    pub fn set_still_in(&mut self,is_still_in:bool){
        self.is_still_in = is_still_in;
        if is_still_in {
            self.dir = (self.start_pos - self.position).normalized();
        }
    }
}
