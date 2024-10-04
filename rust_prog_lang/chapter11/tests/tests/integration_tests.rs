use tests::add_two;

#[test]
fn it_add_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
