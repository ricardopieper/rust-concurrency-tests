mod coarse_grained_linked_set;
use coarse_grained_linked_set::CoarseLockLinkedSet;

use std::thread;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use rand::Rng;

fn monitor(set: Arc<CoarseLockLinkedSet<i32>>) -> (JoinHandle<Vec<i32>>, Arc<AtomicBool>) {
    let cancellation_token = Arc::new(AtomicBool::new(false));
    let token = cancellation_token.clone();
    let thread = thread::spawn(move || {
        let mut sizes: Vec<i32> = vec![];
        while !token.load(Ordering::Relaxed) {
            let len = (*set).len();  
            println!("Size: {:?}", len);
            thread::sleep(Duration::from_millis(1000)); 
            sizes.push(len);
        }
        sizes
    });
    (thread, cancellation_token)
}

fn push_values(num_threads:i32, set: Arc<CoarseLockLinkedSet<i32>>)
    -> Vec<(JoinHandle<()>, Arc<AtomicBool>)> {

    (0..num_threads).into_iter().map(|_i| {
        let thread_set = set.clone();
        let cancellation_token = Arc::new(AtomicBool::new(false));
        let token = cancellation_token.clone();
        let thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while !token.load(Ordering::Relaxed) {
                let number = rng.gen_range(1, 10000);
                thread_set.add(number);
            }
            ()
        });
        (thread, cancellation_token)
    }).collect()
}

fn delete_values(num_threads:i32, set: Arc<CoarseLockLinkedSet<i32>>) 
    -> Vec<(JoinHandle<()>, Arc<AtomicBool>)> {
    (0..num_threads).into_iter().map(|_i| {
        let thread_set = set.clone();
        let cancellation_token = Arc::new(AtomicBool::new(false));
        let token = cancellation_token.clone();
        let thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while !token.load(Ordering::Relaxed) {
                let number = rng.gen_range(1, 10000);
                thread_set.remove(number);
            }
            ()
        });
        (thread, cancellation_token)
    }).collect()
}

fn main() {

    let simulation_duration = Duration::from_millis(60000);

    let set = Arc::new(CoarseLockLinkedSet::<i32>::new());

    let mut threads: Vec<(JoinHandle<()>, Arc<AtomicBool>)> = vec![];

    threads.extend(push_values(24, set.clone()));
    threads.extend(delete_values(24, set.clone()));

    let monitor_thread = monitor(set.clone());

    thread::sleep(simulation_duration);

    for (_thread, thread_cancelled) in threads.iter() {
        thread_cancelled.store(true, Ordering::Relaxed);
    }
    
    for (thread, _thread_cancelled) in threads {
        thread.join().unwrap();
    }

    let (monitor_thread, monitor_stop) = monitor_thread;
    monitor_stop.store(true, Ordering::Relaxed);
    let monitored_sizes = monitor_thread.join().unwrap();

    let average = monitored_sizes.iter()
        .fold(0, |acc, x| acc + x) as f64 / monitored_sizes.len() as f64;

    println!("Average number of items: {:?}", average); 
    println!("Inserts: {:?}", set.get_inserts()); 
    println!("Deletes: {:?}", set.get_deletes()); 

}
