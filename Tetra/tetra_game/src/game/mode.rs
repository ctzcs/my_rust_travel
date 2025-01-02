use std::cell::{RefCell, RefMut};
use std::error::Error;
use std::rc::Rc;
use tetra::Context;
pub mod sample_mode;


pub trait IMode {
    fn ui(&mut self, ctx: &mut Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
    fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
    fn event(&mut self, ctx: &mut Context,event: tetra::Event) -> Result<(), Box<dyn Error>>;
}
//#[derive(Clone)]
pub struct Model<T>
    where T:IModelData
{
    pub data: Rc<RefCell<T>>,
}

impl<T> Model<T>
    where T:IModelData,
    //T:IModelData
{
    
    fn get_mut_data(&self) -> RefMut<'_,T>
    { 
        self.data.borrow_mut()   
    }
}

impl<T:IModelData> Clone for Model<T>{
    fn clone(&self) -> Self {
        Model::<T>{
            data:self.data.clone()
        }
    }
}

pub trait IModelData {

}