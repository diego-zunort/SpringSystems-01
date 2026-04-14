use std::sync::{Condvar};

pub struct ZeroEvenOdd {
    n: i32,
    state: Mutex<(i32, bool)>, // (curr, zero_turn)
    cv: Condvar,
}

impl ZeroEvenOdd {
    pub fn new(n: i32) -> Self {
        Self {
            n,
            state: Mutex::new((1, true)),
            cv: Condvar::new(),
        }
    }

    pub fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n {
            let mut lock = self.state.lock().unwrap();
            while !lock.1 {
                lock = self.cv.wait(lock).unwrap();
            }

            print_number(0);

            lock.1 = false; // now it's odd/even's turn
            self.cv.notify_all();
        }
    }

    pub fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        loop {
            let mut lock = self.state.lock().unwrap();

            while lock.1 || lock.0 % 2 != 0 {
                if lock.0 > self.n {
                    return;
                }
                lock = self.cv.wait(lock).unwrap();
            }

            if lock.0 > self.n {
                return;
            }

            print_number(lock.0);

            lock.0 += 1;
            lock.1 = true; // back to zero
            self.cv.notify_all();
        }
    }

    pub fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        loop {
            let mut lock = self.state.lock().unwrap();

            while lock.1 || lock.0 % 2 == 0 {
                if lock.0 > self.n {
                    return;
                }
                lock = self.cv.wait(lock).unwrap();
            }

            if lock.0 > self.n {
                return;
            }

            print_number(lock.0);

            lock.0 += 1;
            lock.1 = true; // back to zero
            self.cv.notify_all();
        }
    }
}