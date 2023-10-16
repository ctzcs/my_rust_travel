mod declare_macro;
use macro_derive;
use macro_derive::HelloMacro;

fn main() {
    let v = my_vec![1,1,2];
    for v in v.iter() {
        println!("Got:{}",v);
    }
    Pancakes::hello_macro();
}
#[derive(HelloMacro)]
struct Pancakes;

//注意，一定要有一个包里面有derive实现的Trait
pub trait HelloMacro{
    fn hello_macro();
}
