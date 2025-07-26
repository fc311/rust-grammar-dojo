#[cfg(test)]
mod tests {
    use blog_post_workflow::Post;

    #[test]
    fn test_new_post_is_draft() {
        let post = Post::new();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_add_text_to_draft() {
        let mut post = Post::new();
        post.add_text("Hello, Rust!");
        assert_eq!(post.content(), ""); // Content still empty in Draft
    }

    #[test]
    fn test_request_review() {
        let mut post = Post::new();
        post.add_text("Hello, Rust!");
        post.request_review();
        assert_eq!(post.content(), ""); // Still empty in PendingReview
    }

    #[test]
    fn test_approve_post() {
        let mut post = Post::new();
        post.add_text("Hello, Rust!");
        post.request_review();
        post.approve();
        assert_eq!(post.content(), "Hello, Rust!");
    }
}
