use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// Creating an Abstraction of Behavior with Closures

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn creating_an_abstraction_of_behavior_with_closures() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_1(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout_1(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// Refactoring Using Functions

fn refactoring_using_functions() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_2(simulated_user_specified_value, simulated_random_number);
}
fn generate_workout_2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// Refactoring with Closures to Store Code

fn refactoring_using_closures_to_store_code() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_3(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout_3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// Closure Type Inference and Annotation

fn generate_workout_4(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn closure_type_inference_and_annotation_1() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    add_one_v3(1);
    let add_one_v4 = |x| x + 1;
    add_one_v4(1);
}

fn closure_type_inference_and_annotation_2() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// Storing Closures Using Generic Parameters and the Fn Traits

struct Cacher1<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher1<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher1<T> {
        Cacher1 {
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

fn generate_workout_5(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher1::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn storing_closures_using_generic_parameters_and_the_fn_traits() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_5(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests_1 {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher1::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}

struct Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher2<T> {
        Cacher2 {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(value) => value.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher2::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}

// Capturing the Environment with Closures

fn capturing_the_environment_with_closures() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

fn main() {
    capturing_the_environment_with_closures();
}
