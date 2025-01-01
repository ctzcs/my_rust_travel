use crate::entity::IEntity;
use crate::game::mode::IModel;

pub struct EntityManager<T:IModel> {
    entities:Vec<Box<dyn IEntity<T>>>
}

impl<T:IModel> EntityManager<T>{
    pub fn new(heroes:Vec<Box<dyn IEntity<T>>>) -> EntityManager<T> {
        EntityManager {
            entities: heroes
        }
    }
    
    pub fn add(&mut self,entity: Box<dyn IEntity<T>>){
        self.entities.push(entity);
    }
    
    pub fn remove(&mut self,entity: Box<dyn IEntity<T>>){
        
        if let Some(index) = self.entities.iter().position(|item| item.get_id() == entity.get_id()){
            self.entities.remove(index);
        }
    }
    
    pub fn get_all(&mut self) -> &mut Vec<Box<dyn IEntity<T>>> {
        &mut self.entities
    }
}
