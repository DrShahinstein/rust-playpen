use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

const NUM_PRODUCERS: usize = 4;
const NUM_VALUES_PER_PRODUCER: usize = 10;

fn producer_task(id: usize, data: Arc<Mutex<Vec<i32>>>, sender: mpsc::SyncSender<()>) {
    for _ in 0..NUM_VALUES_PER_PRODUCER {
        let value = rand::random::<i32>() % 100;
        println!("Producer {} generated: {}", id, value);

        let mut data = data.lock().unwrap();
        data.push(value);
    }

    sender.send(()).unwrap();
}

fn consumer_task(data: Arc<Mutex<Vec<i32>>>, receiver: mpsc::Receiver<()>) {
    let mut sum = 0;

    for _ in 0..NUM_PRODUCERS {
        match receiver.recv() {
            Ok(_) => {
                let data = data.lock().unwrap();
                for value in &*data {
                    sum += *value;
                }
            }
            Err(_) => {
                println!("No data received. Waiting...");
            }
        }
    }

    println!("Sum of all values: {}", sum);
}

pub fn run() {
    let data = Arc::new(Mutex::new(Vec::new()));
    let (sender, receiver) = mpsc::sync_channel(NUM_PRODUCERS); // Use a bounded channel

    let mut producer_handles = vec![];
    for i in 0..NUM_PRODUCERS {
        let data_clone = Arc::clone(&data);
        let sender_clone = sender.clone();

        let handle = thread::spawn(move || {
            producer_task(i, data_clone, sender_clone);
        });

        producer_handles.push(handle);
    }

    let consumer_handle = thread::spawn(move || {
        consumer_task(data, receiver);
    });

    for handle in producer_handles {
        handle.join().unwrap();
    }

    consumer_handle.join().unwrap();
}
