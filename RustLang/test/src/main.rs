use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    let a = A{x:1,y:2};
    a.to_string_my();
    a.show();
    let b = re(&1,&2);
    println!("Hello, world!{}",b);
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

impl<T> Show for A<T>
{
    fn show(&self) {
        println!("实现的show")
    }
}

impl<T> MyTrait for T where T:Show
{
    fn to_string_my(&self) {
        println!("第二个实现");
    }
}