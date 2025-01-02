pub mod character;
pub mod id;
pub mod manager;
mod building;
use std::any::Any;
use tetra::Context;
use crate::game::mode::{IModelData, Model};

pub trait IEntity<T: IModelData>{
    fn get_id(&self) -> &u32;
    
    fn update(&mut self, ctx:&mut Context, model: Model<T>);
    fn draw(&mut self,ctx:&mut Context);
    
}

#[derive(PartialEq,Debug)]
pub struct BaseEntity{
    id:u32,
}







