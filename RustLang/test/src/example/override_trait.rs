pub fn override_trait_example() {
    let p = Parent;
    p.do_something();
}

//目前认为这种使用组合的方式会更好
pub fn override_trait_example2() {
    let sub_parent = SubParent{ base:DefaultParent,name:"sub_parent".to_string() };
    sub_parent.do_something();
}


///1. 使用单独的默认方法
pub trait IParent{
    fn do_something(&self){
        self.seal_do_default();
    }

    fn seal_do_default(&self){
        println!("Call from IParent!");
    }
}

pub struct Parent;
impl IParent for Parent {
    ///在trait中写一些默认实现，这些实现期望是不被重写，但是会被子类调用
    /// 写一些实际的实现，子类可以重写的
    fn do_something(&self) {
        self.seal_do_default();
        println!("Call from Parent");
    }
}

///2. 还能使用组合
trait IParent1{
    fn do_something(&self){
        println!("call from IParent");
    }
}
struct DefaultParent;
impl IParent1 for DefaultParent {}

struct SubParent{
    base:DefaultParent,
    name:String,
}

impl IParent1 for SubParent{
    fn do_something(&self){
        self.base.do_something();
        println!("call from SubParent");
    }
}

//3. 还能嵌套trait,我觉得可能不好
//就是再创建一个trait，去覆盖trait的默认实现