use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, time};

fn main() {
    fn timeout_thread_result<'a, T>(
        counter: &'a Arc<Mutex<T>>,
        handle: thread::JoinHandle<()>,
    ) -> Option<MutexGuard<'a, T>> {
        let t0 = time::Instant::now();
        let mut deltat = time::Instant::now() - t0;
        while deltat < time::Duration::from_secs(5) {
            // longer wait than the 3 secs below
            let guard = counter.try_lock();
            match guard {
                Ok(guard) => {
                    if handle.is_finished() {
                        return Some(guard);
                    }
                }
                Err(_) => {
                    deltat = t0 - time::Instant::now();
                    continue;
                }
            };
        }
        None
    }

    let counter = Arc::new(Mutex::new(0));

    let counter1 = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(3)); // makes sure second thread acquires lock before this one.
        let mut num = counter1.lock().unwrap();

        *num += 1;
    });

    let counter2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        let prev = timeout_thread_result(&counter2, handle1);
        match prev {
            Some(mut counter) => *counter += 1,
            None => {}
        }
    });

    handle2.join().unwrap();

    println!("Result: {}", *counter.lock().unwrap());
}
