pub mod concurrency {
    use std::thread;

    pub fn creating_threads() {
        let outer_scope = 412;

        let join_handle = thread::spawn(move || {
            outer_scope * 2
        });

        let result = join_handle.join();
        match result {
            Ok(value) => {
                println!("{}", value);
            }
            Err(_) => {}
        }
    }

    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    pub fn thread_communications() {
        let (john_tx, john_rx) = mpsc::channel();
        let (sarah_tx, sarah_rx) = mpsc::channel();

        let john_handle = thread::spawn(move || {
            john_chat(sarah_tx, john_rx);
        });

        let sarah_handle = thread::spawn(move || {
            sarah_chat(john_tx, sarah_rx);
        });
        
        match john_handle.join() {
            Ok(_) => {}
            Err(_) => {}
        }
        
        match sarah_handle.join() {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    fn sarah_chat(john_tx: Sender<&str>, sarah_rx: Receiver<&str>) {
        let result = sarah_rx.recv();
        // `unwrap()` will panic if the `result` Enum is empty
        // Should use a match statement to handle the situation
        println!("{}", result.unwrap());

        let _send_result = john_tx.send("Hello John");
    }

    fn john_chat(sarah_tx: Sender<&str>, john_rx: Receiver<&str>) {
        let _send_result = sarah_tx.send("Hello Sarah.");
        let result = john_rx.recv();

        println!("{}", result.unwrap());
    }
}