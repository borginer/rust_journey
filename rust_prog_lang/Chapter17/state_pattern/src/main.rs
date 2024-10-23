use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("very good piece of work");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    post.approve();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("very good piece of work", post.content());

    post.reject();
    assert_eq!("very good piece of work", post.content());

    println!("finished");
}
