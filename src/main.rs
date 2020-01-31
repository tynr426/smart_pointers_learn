mod my_box;
mod my_rc;
use my_box::MyBox;
mod ref_cell;
mod thread_mutex;
mod channel_send_rec;
use ref_cell::{LimitTracker, MockMessenger};
use std::cell::RefCell;
use thread_mutex::{thread_mutex,dead_lock};
use channel_send_rec::channel_send_rec;
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); //*y=*(y.deref())
    let xx = MyBox { value: "Rust" };
    assert_eq!("Rust", *xx);
    let m = MyBox::new(String::from("Rust"));
    //解引用强制多态所以能成功，标准库中提供了 String 上的 Deref 实现，有两次解引用
    //一个是MyBox,一个是String
    hello(&m);//=hello(&(*m)[..]);
    assert_eq!(*xx, *m);
    my_rc::test();
    it_sends_an_over_75_percent_warning_message();
    //消息传递
    channel_send_rec();
    //mutex互斥，arc+mutex 多线程间共享所有权
    //thread_mutex();

}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    let f: std::cell::Ref<'_, std::vec::Vec<std::string::String>> =
        mock_messenger.sent_messages.borrow();
    println!("{:?}", f);
    println!(
        "{:?},{:?}",
        mock_messenger.sent_messages.borrow(),
        (mock_messenger.sent_messages)
    );
}
