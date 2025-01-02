use crate::game::mode::IModelData;

pub mod move_method;



pub trait ICommand<T: IModelData>
{
    fn execute(&self,model: &mut T);
    fn undo(&self,model: &mut T);
}


struct CommandQueue<T: IModelData>{
    commands:Vec<Box<dyn ICommand<T>>>
}

impl<T: IModelData> CommandQueue<T>{

    fn push(&mut self, command: Box<dyn ICommand<T>>) {
        self.commands.push(command);
    }
    
    fn execute_all(&mut self, model: &mut T) {
        for command in self.commands.drain(..) {
            command.execute(model);
        }
    }

}



