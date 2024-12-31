use tetra::math::Vec2;

///速度和位置模块
#[derive(PartialEq,Debug)]
pub struct VelPos
{
    position:Vec2<f32>,
    velocity:Vec2<f32>,
}

impl VelPos {
    pub fn get_position(&self)->&Vec2<f32>{
        &self.position
    }
    pub fn set_position(&mut self,new_pos:Vec2<f32>){
        self.position = new_pos;
    }

    ///获取速度
    pub fn get_vel(&self)->&Vec2<f32>{
        &self.velocity
    }
    pub fn set_vel(&mut self,new_vel:Vec2<f32>){
        self.velocity = new_vel;
    }
    pub fn new(position: Vec2<f32>,velocity:Vec2<f32>)->VelPos{
        VelPos{
            position,
            velocity,
        }
    }
}