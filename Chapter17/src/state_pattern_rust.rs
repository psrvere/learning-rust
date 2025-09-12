
/*
    In state_pattern example we implemented state pattern as done in object oriented programming
    languages. This way of implementing doesn't make use of all of the Rust strengths

    So let's implement it a slightly different way where we will encode blog post workflow
    into a type system. After implementing following examples:
    - The implementation doesn't follow the state design pattern of OOPs anymore
    - the transformations between the states are not longer encapsulated
    But, invalid states are not impossible because of checking at compile time
*/


pub fn blog_example() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // following code will not compile as DraftPost instance doesn't implement
    // content method
    // assert_eq!("", post.content());

    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

// This represents Published Post
pub struct Post {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

// This represents Draft Post
pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

// No content method define in this implementation
impl PendingReviewPost {
    // This method takes ownership of self i.e. it consumes PendingReviewPost
    // and transforms it into Post
    // Also, this is the only way to get Published Post instance
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
}

// This doesn't implement content method
// So any attempt of printing content on DraftPost will be caught by compiler
// i.e. we are implementing states as different types
// rather than encapsulating the states and transitions completely in previous example
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // This method takes ownership of self i.e. it consumes DraftPost
    // and transforms it into PendingReviewPost
    // Also, this is the only way to get PendingReviewPost instance
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

