use std::cell::{Cell, RefCell};
fn main() {

    let spider = SpiderRobot{error_count:Cell::new(0),normal_count:0,file:RefCell::new(String::from("我的文件\n"))};
    println!("{0}",spider.has_errors());
    println!("{0}",spider.error_count.get());
    spider.add_error();
    println!("{0}",spider.error_count.get());
    //比如这里，如果要传入mut，但是声明的是不可变的，不可变的怎样都没法改
    //let mut s = &mut spider;
    spider.log("add a msg!\n");
    //所以只能通过借用refcell中的元素
    println!("{0}",spider.file.borrow());
}

//内部可变性
pub struct SpiderRobot{
    error_count:Cell<u32>,
    normal_count:u32,
    file:RefCell<String>
}

impl SpiderRobot {

    //这个normal方法行不通,违反了编译器规则,spider是不可变的
    pub fn  add_normal(&mut self){
        self.normal_count += 1;
    }
    pub fn  add_error(&self){
        let n = self.error_count.get();
        self.error_count.set(n + 1);
    }
    pub fn has_errors(&self)->bool{
        self.error_count.get() > 0
    }

    pub fn log(&self,message:&str){
        //利用borrow_mut来写入，可能会panic
        {
            let mut result1 = self.file.borrow_mut();
            result1.push_str(message);
        }
        //利用try_borrow_mut来借用
        {
            let result = self.file.try_borrow_mut();
            let mut file = match result {
                Ok(file)=>file,
                Err(error)=>panic!("Problem opening the file: {:?}", error),
            };
            file.push_str(message);
        }
    }
}
