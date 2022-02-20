// Dereferencing a Raw Pointer

fn dereferencing_a_raw_pointer_1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

fn dereferencing_a_raw_pointer_2() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

fn dereferencing_a_raw_pointer_3() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// Calling an Unsafe Function or Method

unsafe fn dangerous() {}

fn calling_an_unsafe_function_or_method() {
    unsafe {
        dangerous();
    }
}

// Creating a Safe Abstraction over Unsafe Code

fn creating_a_safe_abstraction_over_unsafe_code_1() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn creating_a_safe_abstraction_over_unsafe_code_2() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

// Using extern Functions to Call External Code

extern "C" {
    fn abs(input: i32) -> i32;
}

fn using_extern_functions_to_call_external_code() {
    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
}

// Accessing or Modifying a Mutable Static Variable

static HELLO_WORLD: &str = "Hello, world!";

fn accessing_or_modifying_a_mutable_static_variable_1() {
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn accessing_or_modifying_a_mutable_static_variable_2() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Implementing an Unsafe Trait

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    using_extern_functions_to_call_external_code();
}
