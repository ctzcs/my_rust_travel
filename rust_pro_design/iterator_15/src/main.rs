mod iter_adapter;
mod create_iter;

fn main() {
    let v1 = vec![1,2,3];
    create(v1);
}
///创建迭代器
fn create(mut v1:Vec<i32>){
    //create_iter::new_iter(&v1);
    //create_iter::new_iter1(&v1);
    create_iter::new_iter_mut(&mut v1);
    // create_iter::new_into_iter(&v1);
    create_iter::new_into_iter1(&mut v1);
    // create_iter::new_into_iter2(v1);
    v1.remove(1);
}

