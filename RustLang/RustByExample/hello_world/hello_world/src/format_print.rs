use std::fmt::{Display, Formatter, write};

//将格式化文本写到字符串
//format!("");
//print! 将文本输出到控制台
//println! 结尾追加换行符
//eprint! 将文本输出到标准错误
//eprintln! 结尾追加换行符
pub fn main(){
    println!("{}days/month",31);
    let month:i32 = 3;
    println!("{month}");
    println!("{month:>width$}",month = 1,width = 2);
    debug_sample();
    println!("_________");
    display_sample();
    println!("_________");
    list_sample();
}
#[derive(Debug)]
struct DebugPrintable(i32);
#[derive(Debug)]
struct WrapDebug(DebugPrintable);

///1. 所有的std天生可以用{:?}打印
/// 2. 美化打印{:#?}
fn debug_sample(){
    let printable_num = DebugPrintable(2);
    println!("{:?}", printable_num);

    let wrap_debug = WrapDebug(printable_num);
    println!("{:?}",wrap_debug);
    println!("{:#?}",wrap_debug);
}

///实现display trait
struct DisplayStructure(i32);

impl Display for DisplayStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //和println类似
        write!(f, "{}", self.0)
    }
}

fn display_sample(){
    let s = DisplayStructure(4);
    println!("{s}");
}


struct List(Vec<i32>);
impl Display for List{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //获取vec
        let vec = &self.0;
        write!(f,"[")?;
        //迭代vec,获取迭代次数和下一个值
        for (count,v) in vec.iter().enumerate() {
            if count != 0 { write!(f,",")?;  }
            write!(f,"{}",v)?;
        }
        write!(f,"]")
    }
}

fn list_sample(){
    let list = List(vec![1,2,3,4,5]);
    println!("{list}");
}