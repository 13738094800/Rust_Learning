use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numbers!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        /*
        *  let语句来创建变量
        *  let apples = 5; //不可变
        *  let mut apples = 5; //可变
        *  new函数创建了一个新的 String 类型空字符串
        */
        let mut guess = String::new();

        /*
        *  调用io库中函数stdin 处理用户输入
        *  &表示一个引用
        *  允许多处代码访问同一处数据，而无需在内存中多次拷贝
        *  如果io::Result实例的值是Err expect会导致程序崩溃并返回
        *  传递给其的参数
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //parse返回一个Result类型,result拥有Ok、Err成员
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    
}
