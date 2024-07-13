use std::collections::HashMap;
use std::hash::Hash;
use lazy_static::lazy_static;
use tetra::graphics::Texture;

pub const PANEL:&[u8] = include_bytes!("../../resources/white_man.png");
pub const MOUSE:&[u8] = include_bytes!("../../resources/mouse.png");
pub const CIRCLE_ANIM:&[u8] = include_bytes!("../../resources/circle_anim.png");


pub struct ResSystem{
    assets:HashMap<i32,Texture>,
}
lazy_static!{
    
}