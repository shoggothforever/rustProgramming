use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
    let _ans = rand::thread_rng().gen_range(1..=100);
    let emo ="U+1F605";
    println!("{}",emo);
    loop{
    let mut guess=String::new();
    println!("input your guess number");
    io::stdin().read_line(&mut guess).expect("输入失败");
    let guess: u32 =match guess.trim().parse() {
        Ok(num)=>num,
        Err(_) => {
            println!("字符串转换成数字类型失败");
            continue;
        },
    };
    println!("your guessed: {guess}");
        match guess.cmp(&_ans){
            Ordering::Greater => println!("猜测数字大了{}",guess-_ans),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
            Ordering::Less => println!("猜测数字小了{}",_ans-guess),
        }
    }
}

