/*          TASK 1

fn main() {
    let operation = |a: i32, b: i32| {
        return a * b;
    };

    println!("Result: {}", operation(10, 5));

    
}*/

/*          TASK 2

fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 1;
        println!("tracker: {}", tracker);
    };

    update();
    update();
}

fn main() {
    track_changes();
}*/


/*      task 3
// Using map + collect
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

// Using for loop
fn process_vector_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];
    let doubled = process_vector(numbers.clone(), |x| x * 2);
    let replaced = process_vector(numbers, |x| if x > 2 { 0 } else { x });
    println!("Doubled: {:?}", doubled);   // [2, 4, 6]
    println!("Replaced: {:?}", replaced); // [1, 2, 0]
}*/

use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let v = (self.computation)();
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}