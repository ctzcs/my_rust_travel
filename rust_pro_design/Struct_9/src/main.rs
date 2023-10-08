use std::cell::Cell;

fn main() {
    let spider = SpiderRobot{error_count:Cell::new(0),normal_count:0};
    println!("{0}",spider.has_errors());
    println!("{0}",spider.error_count.get());
    spider.add_error();
    println!("{0}",spider.error_count.get());
    //比如这里，如果要传入mut，但是声明的是不可变的，不可变的怎样都没法改
    //let mut s = &mut spider;

}

//内部可变性
pub struct SpiderRobot{
    error_count:Cell<u32>,
    normal_count:u32
}

impl SpiderRobot {

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
}
