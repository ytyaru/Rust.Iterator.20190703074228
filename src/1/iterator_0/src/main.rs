fn main() {
    let v = vec![1,2,3];
    let i = v.iter();
    println!("{:?}", i); // Iter([1, 2, 3])
    for value in v { println!("{}", value); } // 1, 2, 3
}
