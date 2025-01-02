use std::any::{Any, TypeId};
use std::collections::HashMap;
use tetra::Context;
use crate::entity::IEntity;
use crate::game::mode::{IModelData, Model};

pub struct EntityManager<T>
where T:IModelData,
      //T:Clone
{
    entities:HashMap<TypeId,Vec<Box<dyn IEntity<T>>>>,
    
}

impl<T> EntityManager<T>
where T:IModelData, 
    //T:Clone
      
{
    pub fn new(entities:HashMap<TypeId,Vec<Box<dyn IEntity<T>>>>) -> EntityManager<T> {
        EntityManager {
            entities
        }
    }
    
    pub fn add<TEntity>(&mut self, entity: Box<dyn IEntity<T>>)
        where TEntity : IEntity<T>,
              TEntity: 'static
    {
        self.entities.entry(TypeId::of::<TEntity>())
            .or_default().push(entity);
        
    }
    
    pub fn remove<TEntity>(&mut self,entity: Box<dyn IEntity<T>>)
        where TEntity:IEntity<T>,
            TEntity:'static
    {
        let id = entity.get_id();
        self.entities.get_mut(&TypeId::of::<TEntity>()).unwrap()
            .retain(|item| item.get_id() != id);
    }

    pub fn get_entities<TEntity>(&mut self) -> Option<&mut Vec<Box<dyn IEntity<T>>>>
        where TEntity : IEntity<T>,
              TEntity: 'static
    {
        self.entities.get_mut(&TypeId::of::<TEntity>())
    }
    
    
    pub fn get_all(&mut self) -> &mut HashMap<TypeId,Vec<Box<dyn IEntity<T>>>> {
        &mut self.entities
    }
    
    pub fn update(&mut self,ctx:&mut Context,model:Model<T>){
        for (_,value) in self.entities.iter_mut() {
            for entity in value {
                entity.update(ctx,model.clone());
            }
        }
    }


    pub fn draw(&mut self,ctx:&mut Context){
        for (_,value) in self.entities.iter_mut() {
            for entity in value {
                entity.draw(ctx);
            }
        }
    }
}
