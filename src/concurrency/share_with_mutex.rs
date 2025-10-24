use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn mem_share_mutex() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let n = 10;
    for _ in 0..n {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    *counter.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mem_share_mutex() {
        let n = mem_share_mutex();
        assert_eq!(n, 10);
    }
}
