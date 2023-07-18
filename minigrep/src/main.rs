use std::{env,process};
use minigrep::FileHead;
//cargo  run word.txt gentle 0 2> error.txt 1 > output.txt
fn main() {
    let args:Vec<String>=env::args().collect();
    let fh=FileHead::new(&args).unwrap_or_else(|err|{
        eprintln!("发生了错误:{:?},请按照如下格式输入信息 [file_name] [regex]",err);
        process::exit(1);
    });
    if let Err(e)=minigrep::run(&fh){
        eprintln!("应用发生了错误:{:?}",e);
        process::exit(1);
    }
}
