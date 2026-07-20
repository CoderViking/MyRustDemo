use std::thread;
use std::time::Duration;


pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number)
}

// 模拟一个耗时计算任务
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    // 线程阻塞2秒钟
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // 声明一个闭包
    // let expensive_closure = |num|{
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_closure = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
    });

        if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        // simulated_expensive_calculation(intensity);
        println!("Next, do {} pushups!", expensive_closure.value(intensity));
        // simulated_expensive_calculation(intensity);
    }else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            // println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::close_package::{Cacher, Counter};

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn close_package() {
        // let x = 4;
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("{:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }

    #[test]
    fn test_iter() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("The value is: {}", val);
        }
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let sum: i32 = v1_iter.sum();
        assert_eq!(sum, 6);

        let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v3, vec![2, 3, 4]);
    }
    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x|x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter{count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}