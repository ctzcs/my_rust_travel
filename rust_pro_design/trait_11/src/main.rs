fn main() {
    let mut vec:Vec<Box<dyn Event>> = Vec::new();
    add_event::<LevelUpEvent>(LevelUpEvent{name:String::from("LevelUpEvent!")}, &mut vec);
    add_event::<SpawnEvent>(SpawnEvent{name:String::from("SpawnEvent!")}, &mut vec);
    for item in &vec {
        item.print()
    }
    
}

//不适用泛型
pub trait Event {
    fn print(&self);
}
//升级事件
struct LevelUpEvent
{
    name:String
}
impl Event for LevelUpEvent{
    fn print(&self) {
        println!("this is {0}",&self.name)
    }
}
//出生事件
struct SpawnEvent
{
    name:String
}
impl Event for SpawnEvent{
    fn print(&self) {
        println!("this is {0}",&self.name)
    }
}
fn add_event<T:Event + 'static>( value:T,vec:&mut Vec<Box<dyn Event>>)
{
    vec.push(Box::new(value))
}

//使用泛型
pub trait Event2<T>{
    type Name;
    fn print(&self);
}

impl Event2<LevelUpEvent> for LevelUpEvent{
    type Name = String;
    fn print(&self) {
        println!("this is {0}",&self.name)
    }
}