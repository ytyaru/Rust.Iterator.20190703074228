fn main() {
    let v = vec![1,2,3];
//    v.iter().map(|x| x + 1); // warning: unused `std::iter::Map` that must be used
//    let v2 = v.iter().map(|x| x + 1).collect(); // error[E0282]: type annotations needed
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    /*
    let i = v.iter();
    let s:i32 = i.sum();
    assert_eq!(s, 6);
//    println!("{:?}", i); // error[E0382]: borrow of moved value: `i`
    println!("{:?}", v);
    */
}
