use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;
use std::thread;

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

const LOCKED: bool = true;
const UNLOCKED: bool = false;

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Mutex {
            lock: AtomicBool::new(UNLOCKED),
            data: UnsafeCell::new(t)
        }
    }

    // spin lock
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.lock.compare_exchange_weak(
            UNLOCKED, LOCKED, 
            Ordering::Acquire, 
            Ordering::Relaxed).is_err(){
                thread::park();
            }
        self.lock.store(LOCKED, Ordering::Relaxed);
        let ret = f(unsafe {&mut *self.data.get()});
        self.lock.store(UNLOCKED, Ordering::Release);
        ret
    }
}

fn main() {
    let lck: &'static _= Box::leak(Box::new(Mutex::new(0)));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle  = thread::spawn(move || {
            for _ in 0..10 {
                lck.with_lock(|v| {
                    *v += 1;
                });
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(lck.with_lock(|v| *v), 10*10);
}
