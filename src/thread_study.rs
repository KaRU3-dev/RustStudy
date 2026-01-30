use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn new_task() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{}", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // スレッドの終了を待つ

    println!("Hello, world!");
}

pub fn new_task_2() {
    // moveを使うことで、親スコープ内の変数にアクセスできる
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("{}", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    println!("Hello, world!");
}

struct Position3D(f64, f64, f64);

pub fn lock_position() {
    let position = Mutex::new(Position3D(0.0, 0.0, 0.0)); // Mutexでラップすることで、他スレッドから変数をロックできる

    {
        let mut position = position.lock().unwrap();
        *position = Position3D(1.0, 2.0, 3.0);
    }

    println!("{:?}", position);
}

pub fn share_multi_thread() {
    let counter = Arc::new(Mutex::new(0)); // Mutexでラップすることで、他スレッドから変数をロックできる
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
