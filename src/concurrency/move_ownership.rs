use std::thread;

async fn move_ownership_to_async_block() {
    let x = vec![1, 2, 3];

    let fut = async move {
        println!("x = {:?}", x);
    };
    // println!("x = {:?}", x); // Error! borrow moved value
    tokio::spawn(fut).await.unwrap();
}

fn move_ownership_to_thread() {
    let val = String::from("Hello World");
    thread::spawn(move || {
        println!("val = {}", val);
    });

    // println!("Gone {val}");  // Error! borrow moved value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_move_ownership_to_async_block() {
        move_ownership_to_async_block().await;
    }
    #[test]
    fn test_move_ownership_to_thread() {
        move_ownership_to_thread();
    }
}
