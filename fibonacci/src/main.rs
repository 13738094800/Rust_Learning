use std::io;

fn fib(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 1..n {
        let next = prev.checked_add(curr)?;
        prev = curr;
        curr = next;
    }
    Some(curr)
}

fn main() {
    println!("你要知道第几个斐波那契数：");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");

    let n: u64 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("请输入一个合法的非负整数");
            return;
        }
    };

    match fib(n) {
        Some(result) => {
            println!("fib({}) = {}", n, result);
        }
        None => {
            println!("结果已超出 u64 能表示的范围（最大只支持到 fib(92)）");
        }
    }
    
}
