
use chapter_567::{lib_demo,tuple_struct};
use chapter_567::Gamer;
use chapter_567::Ts;
#[derive(Debug)]
enum Ms {
    Vessel(Gamer),
    Ultra(Ts),
    Dsm,
    Wusa,
    Yith,
    Nyarla
}
impl Ms {
    fn print(&self){
        println!("from enum:{:#?}",self);
    }
}
fn matcher(ms :&Ms){
        match ms{
            //关联到枚举的内部数据
            // Ms::Ultra=> ms.print(),
            Ms::Vessel(Gamer) =>ms.print(),
            _ =>{
                print!("string_view");
                ms.print()
            },
        }
}
fn main() {
    let gamer1=Gamer{
        fav:String::from("celeste"),
        nick_name:String::from("madeline"),
        level:202,
        exp:500,
    };
    let gamer2=Gamer{
        level:195,
        exp:400,
        fav:String::from("celeste"),
        nick_name:String::from("madeline"),
    };
    println!("{}",gamer1.meet(&gamer2));
    let mg = Ms::Vessel(Gamer{
        fav: "hollow".to_string(),
        nick_name: "abyss".to_string(),
        level: 5,
        exp: 20,
    });
    mg.print();
    let t=Ms::Ultra(Ts(String::from("1"), 2, 3));
    t.print();
    matcher(&Ms::Dsm);
}
