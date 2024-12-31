use std::ops::{Deref, DerefMut};
use crate::entity::hero::SampleCha;
use crate::entity::IEntity;

pub struct EntityManager {
    entities:Vec<Box<dyn IEntity>>
}

impl EntityManager {
    pub fn new(heroes:Vec<Box<dyn IEntity>>) -> EntityManager {
        EntityManager {
            entities: heroes
        }
    }
    
    pub fn add(&mut self,entity: Box<dyn IEntity>){
        self.entities.push(entity);
        
    }
    
    pub fn remove(&mut self,entity: Box<dyn IEntity>){
        
        if let Some(index) = self.entities.iter().position(|item| item.get_id() == entity.get_id()){
            self.entities.remove(index);
        }
    }
    
    pub fn get_all(&mut self) -> &mut Vec<Box<dyn IEntity>> {
        &mut self.entities
    }
    
}
