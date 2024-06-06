use tetra::graphics::Texture;
use tetra::math::Vec2;

pub struct EntityBase {
    pub texture:Texture,
    pub position:Vec2<f32>,
    pub speed:f32,
}
impl EntityBase {
    pub fn new(texture: Texture,position:Vec2<f32>,speed:f32)-> EntityBase {
        EntityBase {texture,position,speed}
    }
}
