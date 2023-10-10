# Rust Deadlock

Following the rust-lang book, I want to save some solutions for later reference. Here I created a
deadlock using shared-memory multithreading. I "solved" the deadlock using a simple algorithm that
release the lock every second before trying to acquire it again.
