///声明宏不用写在另一个crate中
#[macro_export]
//宏名称
macro_rules! my_vec {
    //匹配模式，目的是插入一段代码
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec//返回的东西
        }
    };
}