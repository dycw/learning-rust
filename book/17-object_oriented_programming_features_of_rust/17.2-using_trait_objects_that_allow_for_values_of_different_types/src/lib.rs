// Defining a Trait for Common Behavior

pub trait Draw {
    fn draw(&self);
}

pub struct Screen1 {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen1 {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing the Trait

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
