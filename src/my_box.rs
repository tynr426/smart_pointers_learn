/*
box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能
*/
use std::ops::Deref;
pub struct MyBox<T>{
    pub value:T
}

impl<T> MyBox<T> {
   pub fn new(x: T) -> MyBox<T>{
        MyBox{value:x}
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;//语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式

    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data");
    }
}