// Creating a New Vector

fn creating_a_new_vector_1() {
    let v: Vec<i32> = Vec::new();
}

fn creating_a_new_vector_2() {
    let v = vec![1, 2, 3];
}

// Updating a Vector

fn updating_a_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

// Dropping a Vector Drops Its Elements

fn dropping_a_vector_drops_its_elements() {
    let v = vec![1, 2, 3, 4];
}

// Reading Elements of Vectors

fn reading_elements_of_vectors_1() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn reading_elements_of_vectors_2() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

fn reading_elements_of_vectors_3() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {}", first);
}

// Iterating over the Values in a Vector

fn iterating_over_the_values_in_a_vector_1() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
fn iterating_over_the_values_in_a_vector_2() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
}

// Using an Enum to Store Multiple Types

fn using_an_enum_to_store_multiple_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn main() {
    iterating_over_the_values_in_a_vector_2();
}
