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
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data");
    }
}