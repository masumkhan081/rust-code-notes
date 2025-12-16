fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    #[derive(Debug)]
    struct MyStruct {
        a: i32,
        b: i32,
    }
    let x = MyStruct { a: 10, b: 20 };
    println!("{:#?}", x);

    let a = [1, 2, 3, 4, 5];
    let complete = &a[..];
    println!("{:?}", a);
    println!("{:?}", complete);
}
