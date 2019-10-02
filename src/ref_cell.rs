/**
 * 引用分为可变引用（&mut T）和共享引用（&T），
 * 一个对象上可以创建和同时存在多个共享引用，但看都只能读取对象内容，
 * 不可以修改对象内容。但实际使用中，还是有一些需要通过共享引用来修
 * 改对象内容的场景，比如实现一些逻辑上不修改对象内容但实质上要修改对象内容的方法
 */
use std::cell::RefCell;
pub trait Messenger{
    fn send(&self,msg:&str);
}
pub struct LimitTracker<'a,T:'a+Messenger>{
    messenger:&'a T,
    value:usize,
    max:usize
}
impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a,T> {
        LimitTracker {
            messenger:messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
        self.messenger.send("cccc");
    }
}

    pub struct MockMessenger {
        pub sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        pub fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
