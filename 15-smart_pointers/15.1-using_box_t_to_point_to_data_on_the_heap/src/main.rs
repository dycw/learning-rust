// Using a Box<T> to Store Data on the Heap

fn using_a_box_t_to_point_to_data_on_the_heap() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn main() {
    using_a_box_t_to_point_to_data_on_the_heap()
}
