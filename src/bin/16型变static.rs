fn main() {
    let my_string = String::from("hello world"); // my_string 不是 static 只能活到 第五行代码
    // foo(&my_string); // 报错
    bar(&my_string); // 因为 my_string 本身里面是没有引用的 就满足这个关系 T 是 'static 的一个子类
    // 因为 'static 只约束引用，它不约束非引用的东西
}

fn foo<T>(_input: &'static T) { // &'static 只能代表一种引用 不可变的引用
    println!("foo works");
}


fn bar<T: 'static>(_input: &T) { // 如果T这个类型含有引用的话 他的引用一定比static长或者相等,
    // 这里的 T: 'static 不是要求 _input 这个引用必须是 'static 的，而是要求类型 T 本身满足 'static 生命周期约束。
    // T: 'static 表示：
    // 1、要么 T 不包含任何引用
    // 2、要么 T 包含的引用都是 'static 生命周期的
    // 3、换句话说：T 类型的所有数据要么是自己拥有的，要么引用的数据在整个程序运行期间都有效。
    println!("bar works");
}
