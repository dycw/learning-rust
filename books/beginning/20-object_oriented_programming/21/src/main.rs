fn main() {
    trait Draw {
        fn draw(&self);
    }
    struct Text {
        characters: String,
    }
    impl Text {
        fn from(text: &str) -> Text {
            Text {
                characters: text.to_string(),
            }
        }
    }
    impl Draw for Text {
        fn draw(&self) {
            print!("{}", self.characters);
        }
    }
    struct BoxedText {
        text: Text,
        first: char,
        last: char,
    }
    impl BoxedText {
        fn with_text_and_borders(text: &str, first: char, last: char) -> BoxedText {
            BoxedText {
                text: Text::from(text),
                first: first,
                last: last,
            }
        }
    }
    impl Draw for BoxedText {
        fn draw(&self) {
            print!("{}", self.first);
            self.text.draw();
            print!("{}", self.last);
        }
    }
    let greeting = Text::from("Hello");
    let boxed_greeting = BoxedText::with_text_and_borders("Hi", '[', ']');
    // PRINTING OBJECT SIZES
    use std::mem::size_of_val;
    print!(
        "{} {} {}, {} {} {}, ",
        size_of_val(&greeting),
        size_of_val(&&greeting),
        size_of_val(&&&greeting),
        size_of_val(&boxed_greeting),
        size_of_val(&&boxed_greeting),
        size_of_val(&&&boxed_greeting)
    );
    fn draw_text(txt: &dyn Draw) {
        print!(
            "{} {} {} ",
            size_of_val(txt),
            size_of_val(&txt),
            size_of_val(&&txt)
        );
        txt.draw();
    }
    draw_text(&greeting);
    print!(", ");
    draw_text(&boxed_greeting);
}
