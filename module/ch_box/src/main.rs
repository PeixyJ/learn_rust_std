fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    //从栈移动到堆
    let val = 5;
    let boxed = Box::new(val);
    //从堆移动到栈
    let val = *boxed;
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[test]
fn fn2() {
    //使用Box来存储递归类型
    //因为递归类型的大小是未知的，所以不能直接使用栈来存储
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
