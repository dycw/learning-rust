// Defining Post and Creating a New Instance in the Draft State

pub struct Post1 {
    state: Option<Box<dyn State1>>,
    content: String,
}

impl Post1 {
    pub fn new() -> Post1 {
        Post1 {
            state: Some(Box::new(Draft1 {})),
            content: String::new(),
        }
    }
}

trait State1 {}

struct Draft1 {}

impl State1 for Draft1 {}

// Storing the Text of the Post Content

impl Post1 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

// Ensuring the Content of a Draft Post Is Empty

impl Post1 {
    pub fn content(&self) -> &str {
        ""
    }
}

// Requesting a Review of the Post Changes Its State

pub struct Post2 {
    state: Option<Box<dyn State2>>,
    content: String,
}

impl Post2 {
    pub fn new() -> Post2 {
        Post2 {
            state: Some(Box::new(Draft2 {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State2 {
    fn request_review(self: Box<Self>) -> Box<dyn State2>;
}

struct Draft2 {}

impl State2 for Draft2 {
    fn request_review(self: Box<Self>) -> Box<dyn State2> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State2 for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State2> {
        self
    }
}
