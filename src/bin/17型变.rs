struct Interface<'a> {
    manager: &'a mut Manager<'a>
}

/*
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>   // 'a: 'b 表示 'a 的生命周期必须大于等于 'b (a要获得足够久)
}
 **/

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface<'a> {
        Interface {
            manager: &mut self.manager
        }
    }
}

struct Borrow<'a> {
    s: &'a str
}

impl<'a> Borrow<'a> {
    fn longer<'b>(&self, other: &'b Self) -> &'a str {
    // 比较当前字符串和另一个字符串的长度
        if self.s.len() > other.s.len() {
        // 如果当前字符串更长，返回当前字符串的引用
            self.s
        } else {
        // 否则返回另一个字符串的引用
            other.s
        }
    }
}
#[derive(Debug)]
struct A;

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    // use_list(&list);
}

fn use_list(list: &&List) {
    println!("{}", list.manager.text);
}