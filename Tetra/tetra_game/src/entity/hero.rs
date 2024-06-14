use tetra::math::Vec2;
use crate::entity_component::position::VelPos;

pub enum Hero{
    OldMan(OldMan),
    None
}

#[derive(Debug)]
pub struct OldMan{
    name:String,
    velpos:VelPos,
}

impl OldMan{
    pub fn new(name:String,vel_pos: VelPos)-> OldMan{
        OldMan{
            name:name,
            velpos:vel_pos,
        }
    }

    pub fn update(&mut self){
        self.velpos.set_position(
            self.velpos.get_position() + Vec2::new(1.0,1.0));
        println!("{:?}:{:?}", self.name, self.velpos);
    }
}