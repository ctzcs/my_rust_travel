//iter迭代器的创建和简单使用
#[allow(dead_code)]
pub fn new_iter(v1:&Vec<i32>){
    //迭代器
    //创建一个不可变迭代器
    let v1_iter = v1.iter();
    for val in v1_iter{
        print!("Got:{}\n",val);//这里消耗了这个迭代器
    }
    //这里错误,因为迭代器被创建之后，默认是移动
    // for val in v1_iter {
    //     print!("Got:{}",val);
    // }

    print!("v1[0]:{}",&v1[0]);
}
//iter迭代器使用1:如果想多次使用into()迭代一个集合
#[allow(dead_code)]
pub fn new_iter1(v1:&Vec<i32>){
    //每次使用的时候都创建一个迭代器
    print!("---第一次打印---\n");
    for val in v1.iter() {
        print!("Got:{}\n",val);
    }
    print!("---第二次打印--\n");
    for val in v1.iter() {
        print!("Got:{}\n",val);
    }
}
//使用iter_mut修改数据
#[allow(dead_code)]
pub fn new_iter_mut(v1:&mut Vec<i32>){
    for val in v1.iter_mut() {
        *val = 0;
    }
    //打印
    for val in v1.iter() {
        print!("Got:{}\n",val);
    }
}

//into_iter共享型
#[allow(dead_code)]
pub fn new_into_iter(v1:&Vec<i32>){
    for val in v1.into_iter()
    {
        print!("Got:{}\n",val);
    }
    //使用完后v1仍然存在
    print!("v1[0]:{}",v1[0]);
}
//可变引用 into_iter,传入mut引用
#[allow(dead_code)]
pub fn new_into_iter1(v1:&mut Vec<i32>){
    for val in v1.into_iter()
    {
        *val = 8;
    }
    //修改成功
    print!("v1[0]:{}",v1[0]);
}
//通过值夺走所有权
#[allow(dead_code)]
pub fn new_into_iter2(v1:Vec<i32>){
    for val in v1.into_iter()
    {
        print!("Got:{}\n",val);
    }
    //已经不存在了
    //这一句错误，因为v1已经被消耗了
    //print!("v1:{}",v1[0]);
}