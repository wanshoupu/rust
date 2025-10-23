use std::thread;
use std::time::Duration;

fn spawn_thead() {
    let join_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread echo {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    join_handle.join().unwrap();
}
fn handle_error_match() {
    let join_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread echo {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    match join_handle.join() {
        Ok(_) => println!("Thread finished successfully"),
        Err(_) => eprintln!("Thread panicked!"),
    }
}
fn handle_if_error() {
    let join_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread echo {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    if let Err(e) = join_handle.join() {
        eprintln!("join error: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spawn_thead() {
        spawn_thead();
    }
    #[test]
    fn test_handle_error_match() {
        handle_error_match();
    }
    #[test]
    fn test_handle_if_error() {
        handle_if_error();
    }
}
