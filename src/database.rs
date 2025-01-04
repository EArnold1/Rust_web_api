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

    pub fn add_multi_post(&mut self, input: Vec<PostInput>) -> Vec<Post> {
        let mut list = Vec::new();

        for i in 0..input.len() {
            let post = Post::new(
                input[i].title.as_str(),
                input[i].body.as_str(),
                input[i].author.as_str(),
            );

            self.posts.push(post.clone());
            list.push(post);
        }

        list
    }

    pub fn posts(&self) -> &Vec<Post> {
        &self.posts
    }
}
