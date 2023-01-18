fn main() {
    let mut x = vec![1,2,3];
    
    for num in x.iter_mut() {
        *num *= 2;
    }
    println!("{:?}", x)
}
