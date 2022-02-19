// Running Code on Cleanup with the Drop Trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn running_code_on_cleanup_with_the_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

// Dropping a Value Early with std::mem::drop

fn dropping_a_value_early_with_std_mem_drop() {
    let c = CustomSmartPointer {
        data: String::from("some datak"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn main() {
    dropping_a_value_early_with_std_mem_drop();
}
