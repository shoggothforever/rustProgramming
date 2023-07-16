use std::arch::x86_64::_subborrow_u32;
use std::fs::copy;
use std::path::Component::Prefix;
use std::pin::Pin;
use rand::Rng;
fn main() {
    let mut it=1;
    while it<10{
        let num:u32=rand::thread_rng().gen_range(1..=100);
        let res = num%4;
        match res{
            1=>println!("luckily you got 1"),
            _ => {}
        }
        let res =Some(num%4);
        verify(res);
        it=it+1;
    }
    let mut str="hello world".to_string();
    println!("{}",first_world(&str));
    str.clear();
    doublefree();
}
fn verify(val: Option<u32>) -> Option<u32>{
match val{
    Some(i)=>{
        println!("the result is {i}");
        Some(i)
    },
    None=>None,
}
}
fn doublefree(){

    let  s1=String::from("first");
    let s2=s1;
    println!("{s2}");
    let s3=s2;
    let s4=s3.clone();
    println!("{s3},{s4}")
}
//1:传入String类型,返回usize
//2:返回字符串切片&str,字符串切片是不可变的字面量
//3：传入字符串切片,返回&str,这样可以更加灵活,也不用担心传入String后所有权丢失了
fn first_world(s: &String)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        // println!("{item}");
        if item ==b' ' {
           return &s[..i];
        }
    }
    &s[..]

}