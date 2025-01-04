use crate::{models::Post, PostInput};

#[derive(Clone, Debug)]
pub struct Database {
    posts: Vec<Post>,
}

impl Database {
    pub fn new() -> Database {
        Database { posts: vec![] }
    }

    pub fn add_post(&mut self, input: PostInput) -> Post {
        let post = Post::new(
            input.title.as_str(),
            input.body.as_str(),
            input.author.as_str(),
        );

        self.posts.push(post.clone());

        post
    }

    pub fn posts(&self) -> &Vec<Post> {
        &self.posts
    }
}
