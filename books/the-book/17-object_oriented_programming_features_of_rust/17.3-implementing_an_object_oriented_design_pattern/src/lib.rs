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
        Box::new(PendingReview2 {})
    }
}

struct PendingReview2 {}

impl State2 for PendingReview2 {
    fn request_review(self: Box<Self>) -> Box<dyn State2> {
        self
    }
}

// Adding the approve Method that Changes the Behavior of content

pub struct Post3 {
    state: Option<Box<dyn State3>>,
    content: String,
}

impl Post3 {
    pub fn new() -> Post3 {
        Post3 {
            state: Some(Box::new(Draft3 {})),
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

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State3 {
    fn request_review(self: Box<Self>) -> Box<dyn State3>;
    fn approve(self: Box<Self>) -> Box<dyn State3>;
}

struct Draft3 {}

impl State3 for Draft3 {
    fn request_review(self: Box<Self>) -> Box<dyn State3> {
        Box::new(PendingReview3 {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State3> {
        self
    }
}

struct PendingReview3 {}

impl State3 for PendingReview3 {
    fn request_review(self: Box<Self>) -> Box<dyn State3> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State3> {
        Box::new(Published3 {})
    }
}

struct Published3 {}

impl State3 for Published3 {
    fn request_review(self: Box<Self>) -> Box<dyn State3> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State3> {
        self
    }
}

pub struct Post4 {
    state: Option<Box<dyn State4>>,
    content: String,
}

impl Post4 {
    pub fn new() -> Post4 {
        Post4 {
            state: Some(Box::new(Draft4 {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State4 {
    fn request_review(self: Box<Self>) -> Box<dyn State4>;
    fn approve(self: Box<Self>) -> Box<dyn State4>;
    fn content<'a>(&self, post: &'a Post4) -> &'a str {
        ""
    }
}

struct Draft4 {}

impl State4 for Draft4 {
    fn request_review(self: Box<Self>) -> Box<dyn State4> {
        Box::new(PendingReview4 {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State4> {
        self
    }
}

struct PendingReview4 {}

impl State4 for PendingReview4 {
    fn request_review(self: Box<Self>) -> Box<dyn State4> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State4> {
        Box::new(Published4 {})
    }
}

struct Published4 {}

impl State4 for Published4 {
    fn request_review(self: Box<Self>) -> Box<dyn State4> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State4> {
        self
    }

    fn content<'a>(&self, post: &'a Post4) -> &'a str {
        &post.content
    }
}

// Encoding States and Behavior as Types

pub struct Post5 {
    content: String,
}

pub struct DraftPost5 {
    content: String,
}

impl Post5 {
    pub fn new() -> DraftPost5 {
        DraftPost5 {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost5 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

// Implementing Transitions as Transformations into Different Types

impl DraftPost5 {
    pub fn request_review(self) -> PendingReviewPost5 {
        PendingReviewPost5 {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost5 {
    content: String,
}

impl PendingReviewPost5 {
    pub fn approve(self) -> Post5 {
        Post5 {
            content: self.content,
        }
    }
}
