use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::{self, ErrorKind, Read};
fn learn_map(){
    let keys =vec![100,99,98,97,96];
    let vals=vec![1,2,3,4,5];
    let mut mp:HashMap<_,_>=
        keys.iter().zip(vals.iter()).collect();
    // println!("{}",mp);
    let key=211;
    mp.entry(&key).or_insert(&1);
    println!("{:?}",mp.get_key_value(&key));
    match mp.get(&211){
        Some(val)=>{
            println!("{val}");
        },
        None=>panic!("error")
    }
}
fn learn_panic(){
    use std::fs::File;
    let file=File::open("ferris.jpg").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("ferris.jpg").unwrap_or_else(|error|{
                panic!("Error creating file: {:?}",error);
            })
        }else{
            panic!("Error opening file: {:?}",error);
        }
    });

}
fn open_file_sugar(s:&str)->Result<String,io::Error>{
    use std::fs::File;
    // let mut file=File::open(&s)?;
    // file.read_to_string(&mut word)?;


    let mut word =String::new();
    File::open(&s)?.read_to_string(&mut word)?;
    println!("{:?}",word);
    Ok(word)

}
use std::error::Error;
use std::ptr::null;
trait node<'a,T>{
    fn lesser_node(&'a self, other:&'a Node<T>) ->&'a Node<T>;
}
#[derive(Debug)]
struct Node<'a,T>{
    num :&'a T,
}
impl  <'a,T>Node<'a,T>{
    fn lesser_node(&'a self,other:&'a Node<T>)->&'a Node<T>
    where T:PartialOrd+PartialEq
    {
        if self.num<other.num{
            self
        }else{
            other
        }
    }
}
use std::fmt::Display;
fn larger_node<'a , T>(
    x:&'a Node<'a,T>,
    y:&'a Node<'a,T>
)-> &'a Node<'a,T>
where T:Display +PartialOrd
{
    if x.num<y.num{
        y
    }else{
        x
    }
}

fn test_node(){
    let a=Node{num:&22};
    let b=Node{num:&32};
    println!("{:?}",larger_node(&a,&b));
    println!("{:?}",a.lesser_node(&b));
}
fn main(){
    // test_node();
    learn_map();
}
// fn main() ->Result<(),Box<dyn Error>>{
//     open_file_sugar(&"word.txt")?;
//     Ok(())
//
//
// }