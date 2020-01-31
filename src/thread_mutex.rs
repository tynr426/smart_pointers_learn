use std::sync::{Mutex,Arc};
use std::thread;
pub fn thread_mutex(){
    //多线程间共享所有权
    let counter= Arc::new(Mutex::new(0));
    let handles :Vec<_> =(0..10).map(|_|{
         let counter = Arc::clone(&counter);
        thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            for _ in 0..50000000 {
                

                *num += 1;
            }
            *num
        })
    }).collect();
   
    let xc = handles.len();

    for h in handles{
        println!("Thread finished with count={:?}",h.join().map_err(|_| "Could not join a thread!").unwrap());
    }

    println!("done!{:?}", xc);
    println!("Result: {:?}", counter.lock().unwrap());
}
pub fn dead_lock(){
    let p = Arc::new(Mutex::new(0));
    let q = Arc::clone(&p);
    let th = thread::spawn(move || {
        let mut i = p.lock().unwrap();
        *i+=1;
    });
    //解决方案就是在th.join().unwrap();之前释放掉主线程拿到的锁，因此可以将let mut j = q.lock().unwrap();放入一个代码块
    let mut j = q.lock().unwrap();
    *j+=1;
    th.join().unwrap();
}