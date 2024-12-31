pub mod character;
pub mod id;
pub mod manager;
mod building;

use std::any::Any;
use tetra::Context;
use crate::game::mode::sample_mode::{SampleMode, SampleModeModel};

pub trait IEntity{
    fn get_id(&self) -> &u32;
    fn update(&mut self,ctx:&mut Context,sample_mode_model: &mut SampleModeModel);
    fn draw(&mut self,ctx:&mut Context);
    
}

#[derive(PartialEq,Debug)]
pub struct BaseEntity{
    id:u32,
}







