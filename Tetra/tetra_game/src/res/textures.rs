use std::collections::HashMap;
use std::hash::Hash;
use lazy_static::lazy_static;
use tetra::graphics::Texture;

pub const MOUSE:&[u8] = include_bytes!("../../resources/textures/mouse.png");
pub const CIRCLE_TEXTURE:&[u8] = include_bytes!("../../resources/textures/circle_anim.png");
pub const ROCKET_TEXTURE:&[u8]=include_bytes!("../../resources/textures/rocket.png");

pub struct ResSystem{
    assets:HashMap<i32,Texture>,
}
lazy_static!{
    
}