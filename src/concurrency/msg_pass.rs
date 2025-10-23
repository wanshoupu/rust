use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn msg_pass() {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let val = String::from("Hello World");
        tx.send(val).unwrap();
        // println!("val = {}", val); // Err: trying to use value after being moved
    });

    // let msg = rx.recv().unwrap();
    // println!("Got {msg}");
    for msg in rx {
        println!("{}", msg);
    }
}

fn multiple_msg_pass() {
    let (tx, rx) = mpsc::channel::<String>();
    let vec = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("My"),
        String::from("Name"),
        String::from("Is"),
        String::from("Foo"),
    ];

    thread::spawn(move || {
        for val in vec {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for msg in rx {
        println!("Got {msg}");
    }
}

fn channel_clone() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vec = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("My"),
            String::from("Name"),
            String::from("Is"),
            String::from("Foo"),
        ];
        let mut rng = rand::thread_rng();
        for val in vec {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(rng.gen_range(100..2000)));
        }
    });
    thread::spawn(move || {
        let vec = vec![
            String::from("Apple"),
            String::from("Orange"),
            String::from("Pear"),
            String::from("Pineapple"),
            String::from("Grapefruit"),
            String::from("Kiwi"),
        ];
        let mut rng = rand::thread_rng();
        for val in vec {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(rng.gen_range(100..2000)));
        }
    });

    for msg in rx {
        println!("Got {msg}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_pass() {
        msg_pass();
    }

    #[test]
    fn test_multiple_msg_pass() {
        multiple_msg_pass();
    }

    #[test]
    fn test_channel_clone() {
        channel_clone();
    }
}
