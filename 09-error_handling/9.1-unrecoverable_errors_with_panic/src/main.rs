// Unwinding the Stack or Aborting in Response to a Panic

fn unwinding_the_stack_or_aborting_in_response_to_a_panic() {
    panic!("crash and burn");
}

// Using a panic! Backtrace

fn using_a_panic_backtrace() {
    let v = vec![1, 2, 3];

    v[99];
}

fn main() {
    using_a_panic_backtrace();
}
