fn main() {
    let my_string = String::from("hello world"); // my_string 不是 static 只能活到 第五行代码
    // foo(&my_string); // 报错
    bar(&my_string); // 因为 my_string 本身里面是没有引用的 就满足这个关系 T 是 'static 的一个子类
    // 因为 'static 只约束引用，它不约束非引用的东西
}

fn foo<T>(_input: &'static T) { // &'static 只能代表一种引用 不可变的引用
    println!("foo works");
}

fn bar<T: 'static>(_input: &T) { // 如果T这个类型含有引用的话 他的引用一定比static长或者相等
    println!("bar works");
}
