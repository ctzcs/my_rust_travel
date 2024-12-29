mod example;

use std::collections::HashMap;
use std::fmt::Display;
use crate::example::override_trait::{override_trait_example, override_trait_example2};

fn main() {
    /*let a = A{x:1,y:2};
    a.to_string_my();
    a.show();
    let b = re(&1,&2);
    println!("Hello, world!{}",b);*/
    
    let sub_class = SubClass{
        base:BaseClass{
            id:1,
        }
    };
    println!("{}",sub_class.get_id());
    

    general_example();

    override_trait_example2();
}
//生命周期
//1. 由于有多个引用的时候，编译器不知道哪个会返回，导致编译器对生命周期很懵
//2. 加上<'a>声明参数的声明周期相同
fn re<'a>(x:&'a i32,y:&'a i32)->&'a i32
{
    if x > y {
        x
    } else { y }
}
struct A<T>{
    x:T,
    y:T,
}
pub trait Show{
    fn show(&self){println!("我来show")}
}
pub trait MyTrait {
    fn to_string_my(&self);
}

//给所有的A<T>实现方法
impl<T> Show for A<T>
{
    fn show(&self) {
        println!("实现的show")
    }
}
//给所有的T类型实现，这个T可以是trait
impl<T> MyTrait for T 
    where T:Show
{
    fn to_string_my(&self) {
        println!("第二个实现");
    }
}


//trait 关联类型
struct BaseClass{
    id:i32
}
struct SubClass{
    base: BaseClass
}
trait Id{
    fn get_id()->i32;
}

trait BaseTrait{
    type Base;
    fn get_id(&self)->i32;
}

impl BaseTrait for SubClass {
    type Base = BaseClass;

    fn get_id(&self) -> i32 {
        return self.base.id;
    }
}


pub fn lifetime_example(){
    let b = re(&1,&2);
    println!("Hello, world!{}",b);
}

fn general_example(){
    let a = A{x:1,y:2};
    a.to_string_my();
    a.show();
}

