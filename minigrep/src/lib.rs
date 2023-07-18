use std::{fs};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::env;
use std::env::Args;

pub struct FileHead{
    pub filename:String,
    pub query:String,
    pub sensitive:bool
}
impl FileHead {
    pub fn new(mut args: env::Args) -> Result<FileHead,&'static str> {
        if args.len() < 3 {
            return Err("请按照如下格式输入信息 [file_name] [regex]");
        }
        args.next();
        let filename = match args.next(){
            Some(s)=>s,
            None=>panic!()
        };
        let query = match args.next(){
            Some(s)=>s,
            None=>panic!()
        };;
        let sensitive=match args.next(){
            Some(s)=>{
                if s=="0"{
                    true
                }else{
                    false
                }
            },
            None=>panic!()
        };
        Ok(FileHead{filename,query,sensitive })
    }
}
pub fn run(fh:&FileHead)->Result<(),Box<dyn Error>>{
    let content=fs::read_to_string(&fh.filename)?;
    let res=if fh.sensitive{
        search(&fh.query, &content)
    }else{
        search_insensitive(&fh.query,&content)
    };
    // println!("{:?} {:?}\n{:?}",fh.filename,fh.query,content);
    println!("{:?}",res);
    Ok(())
}

pub fn search<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    // let mut res =Vec::new();
    // for line in content.lines(){
    //     if line.contains(query){
    //         res.push(line)
    //     }
    // }
    // res
    content.lines().filter(|line|line.contains(&query)).collect()
}
pub fn search_insensitive<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    // let mut res =Vec::new();
    // let query=query.to_lowercase();
    // for line in content.lines(){
    //     if line.to_lowercase().contains(&query){
    //         res.push(line)
    //     }
    // }
    // res
    content.lines().filter(|line|line.to_lowercase().contains(&query.to_lowercase())).collect()
}




struct Cache<K,V,F>
where
    K:std::cmp::Eq+ std::hash::Hash+Clone+Copy,
    F:Fn(K)->V,
{
    func:F,
    cache:HashMap<K,V>,
}
impl<K,V,F> Cache<K,V,F>
where
    K:std::cmp::Eq+ std::hash::Hash+Clone+Copy,
    F:Fn(K)->V,
{
    fn new(func: F) ->Self{
        Cache{ func, cache: HashMap::new() }
    }
    fn get(&mut self,key:K)->&V{
        if !self.cache.contains_key(&key){
            let value=(self.func)(key.clone());
            self.cache.insert(key,value);
        }
        self.cache.get(&key).unwrap()
    }
}
#[cfg(test)]
mod tests{
    use super::*;
#[test]
    fn cache_test(){
    let mut c =Cache::new(|c|c*c);
    let res1=c.get(2i32).clone();
    let res2=c.get(3i32).clone();
    assert_eq!(res1, c.get(2).clone());
    assert_eq!(res2, c.get(3).clone());
}

#[test]
fn one_test() {
    let query = "gentle";
    let content = "\
do not
go gentle into
that good night";
    assert_eq!(vec!["go gentle into"],search(query,content));
}
#[test]
fn two_test() {
    let query = "GENTLE";
    let content = "\
do not
go gentle into
go GENTLE into
that good night";
    assert_eq!(vec!["go gentle into","go GENTLE into"],search_insensitive(query,content));
}
}