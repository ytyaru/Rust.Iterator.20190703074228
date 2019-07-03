fn main() {
    let v = vec![1,2,3];
    let mut i = v.iter(); // next()を呼ぶならmutableにする必要がある。さもなくばerror[E0596]: cannot borrow `i` as mutable, as it is not declared as mutable
    println!("{:?}", i); // Iter([1, 2, 3])
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), None);
}
