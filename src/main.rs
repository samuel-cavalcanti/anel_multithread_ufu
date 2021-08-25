use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc};
use std::sync::Mutex;
use std::ops::Add;
use rand;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use std::borrow::Borrow;
use std::thread::JoinHandle;
use std::rc::Rc;


fn random_message(size_message: usize) -> String {
    let char_set = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let message: String = (0..size_message)
        .map(|_| {
            let idx = thread_rng().gen_range(0..char_set.len());
            char_set[idx] as char
        })
        .collect();
    message
}


fn work(rx: Receiver<String>, tx: Sender<String>, thread_id: usize) -> Option<String> {
    for message in rx {
        println!("thread: {} message: {} ", thread_id, message.clone());

        let new_message = change_char(message.clone());

        //Quando todos os caracteres forem maiúsculos, o processo repassa a mensagem e então termina.
        if new_message == message.to_uppercase() {
            tx.send(new_message.clone());
            return Some(new_message);
        } else {
            thread::sleep(Duration::from_secs(1));
            tx.send(new_message.clone());
        }
    }

    None
}

fn change_char(message: String) -> String {
    if let Some(char) = message.chars().find(|c| c.is_lowercase()) {
        return message.replacen(char, &*char.to_uppercase().collect::<String>(), 1);
    }

    message
}


fn create_thread(number_of_threads: usize, current_number_of_threads: usize, rx: Receiver<String>, first_tx: Sender<String>) -> Option<JoinHandle<()>> {
    if current_number_of_threads == number_of_threads {
        let handle = thread::spawn(move || {
            let random_message = random_message(80);
            println!("message: {}", random_message.clone());
            first_tx.send(random_message);
            let result = work(rx, first_tx, current_number_of_threads);

            // println!("Finalizado trabalho {}", result.unwrap())
        });
        return Some(handle);
    }

    let handle = thread::spawn(move || {
        let (tx, another_rx) = mpsc::channel::<String>();
        create_thread(number_of_threads, current_number_of_threads + 1, another_rx, first_tx);
        work(rx, tx, current_number_of_threads);
        // println!("finalizou! {}", current_number_of_threads);
    });

    Some(handle)
}


fn main() {
    let number_of_threads = 30;
    let (tx, rx) = mpsc::channel::<String>();

    let option_handle = create_thread(number_of_threads,
                                      1,
                                      rx,
                                      tx);

    if let Some(handle) = option_handle {
        handle.join();
    }
}


