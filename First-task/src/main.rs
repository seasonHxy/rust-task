// 在 lower 子模块中，添加一个函数来循环打印从 'a'~'z' 之间的所有字符
mod lower {
    pub fn print_all_chars() {
        for c in 'a'..='z' {
            println!("{}", c);
        }
    }
}

// 在 upper 子模块中，添加一个函数来循环打印从 'A'~'Z' 之间的所有字符
mod upper {
    pub fn print_all_chars() {
        for c in 'A'..='Z' {
            println!("{}", c);
        }
    }
}



// 在主模块中，添加一个函数来调用子模块中的函数
fn main() {
    lower::print_all_chars();
    upper::print_all_chars();
}

