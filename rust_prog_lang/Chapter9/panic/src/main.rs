fn main() {
   vec_crash(); 
}

fn _crash() {
    panic!("crash and burn");
}

fn vec_crash() {
    let vec = vec![1, 2, 3];
    vec[55];
}
