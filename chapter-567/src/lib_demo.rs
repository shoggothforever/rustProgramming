//这个tag只对应一个struct的声明
#[derive(Debug)]
pub struct Gamer{
    pub fav:String,
    pub nick_name:String,
    pub level:u32,
    pub exp:u32,
}

//unit-like struct
pub fn meet (gamer1:&Gamer,gamer2:&Gamer)->bool{
    print_gamer(&gamer1);
    gamer2.print();
    gamer1.fav==gamer2.fav
}
pub fn print_gamer(g: &Gamer){
    println!("\x1b[93m{:#?}\x1b[0m",g);
}

//定义结构体方法在impl内部实现,传入&self获得所有权或者可变借用
impl Gamer{
    pub fn print(&self){
        println!("\x1b[93m{:#?}\x1b[0m",self);
    }
    pub fn meet (&self,gamer2:&Gamer)->bool{
        meet(&self,&gamer2)
    }
}