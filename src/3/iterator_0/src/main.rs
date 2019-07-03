fn main() {
    let v = vec![1,2,3];
    let i = v.iter();
    let s:i32 = i.sum();
    assert_eq!(s, 6);
//    println!("{:?}", i); // error[E0382]: borrow of moved value: `i`
    println!("{:?}", v);
}
