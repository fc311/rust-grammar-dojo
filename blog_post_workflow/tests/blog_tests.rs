#[cfg(test)]
mod tests {
    use blog_post_workflow::Post;

    #[test]
    fn test_new_post_is_draft() {
        let post = Post::new();
        assert_eq!(post.content(), "");
    }
}
