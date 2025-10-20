use std::thread;
use std::time::Duration;

fn main() {
    // calling a lib function
    rust_projects::two_sum::Solution::two_sum(vec![], 3);

    let join_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread echo {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("main echo {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    join_handle.join().unwrap();
}
