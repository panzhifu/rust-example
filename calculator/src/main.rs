use evalexpr::*;
use std::io;

fn main() {
    loop {
        println!("请输入数学公式（输入 'q' 退出）:");
        let mut input = String::new();
        io::stdin()
           .read_line(&mut input)
           .expect("读取输入失败");

        let input = input.trim();
        if input == "q" {
            break;
        }

        match eval(input) {
            Ok(result) => println!("结果: {}", result),
            Err(e) => println!("错误: {}", e),
        }
    }
}       
