use std::collections::HashMap;

//event
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event{
    Load,
    Save
}
//subscriber
pub type Subscriber = fn(file_path:String);
//publisher
pub struct Publisher{
    events:HashMap<Event,Vec<Subscriber>>
}
impl Publisher{

    pub fn subscribe(&mut self,event_type:Event,listener:Subscriber){
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }
    pub fn unsubscribe(&mut self,event_type:Event,listener:Subscriber){
        
    }
    pub fn notify(&self,event_type:Event,file_path:String){}
}
