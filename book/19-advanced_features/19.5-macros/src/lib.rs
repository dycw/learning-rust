// Declarative Macros with macro_rules! for General Metaprogramming

fn declarative_macros_with_macro_rules_for_general_metaprogramming_1() {
    let v: Vec<u32> = vec![1, 2, 3];
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn declarative_macros_with_macro_rules_for_general_metaprogramming_2() -> Vec<u32> {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
