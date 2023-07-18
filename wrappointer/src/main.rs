use std::cell::{Ref, RefCell};
use crate::List::{Cons, Nil};
use std::rc::{Rc,Weak};
use std::ops::Deref;
#[derive(Debug)]
struct MBox<T>(T);
impl <T>MBox<T>{
    fn new(x:T)->MBox<T>{
        MBox(x)
    }
}
impl <T>Deref for MBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
enum List{
    Cons(i32,Rc<List>),
    Nil
}
fn get_val(x:&i32){
    println!("get value {}",x);
}
fn test_deref(){
    let x=1;
    let m=MBox::new(x);
    assert_eq!(1,*m);
    get_val(&m);
}
fn test_rc(){
    // let b = Box::new(5);
    // println!("{b}");
    let list=Rc::new(Cons(1,Rc::new(Cons(2,Rc::new(Cons(3,Rc::new(Nil)))))));
    //Rc::clone 只是浅拷贝
    let la=Cons(20,Rc::clone(&list));
    let lb=Cons(30,Rc::clone(&list));
    println!("a reference count = {}",Rc::strong_count(&list))
}
#[derive(Debug)]
struct TreeNode{
    val:i32,
    parent:RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>
}
impl TreeNode{
    fn set_parent(&self,parent: &Rc<TreeNode>){
        *self.parent.borrow_mut()=Rc::downgrade(parent);
    }
    fn insert(&self,child:&Rc<TreeNode>){
        // let par=self.clone()
        // *child.parent.borrow_mut()=Rc::downgrade(&Rc::new(par));
        self.children.borrow_mut().push(Rc::clone(child))
    }
    fn get_children(&self) -> Ref<'_, Vec<Rc<TreeNode>>> {
        return self.children.borrow()
    }
}
fn test_tree_node(){
    let child1=Rc::new(TreeNode{
        val: 1,
        parent: RefCell::new(Default::default()),
        children: RefCell::new(vec![]),
    });
    let child2=Rc::new(TreeNode{
        val: 2,
        parent: RefCell::new(Default::default()),
        children: RefCell::new(vec![]),
    });

    let root=Rc::new(TreeNode{
        val: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    root.insert(&child1);
    root.insert(&child2);
    child1.set_parent(&root);
    print!("root is parent of child1 and child2 :{:#?}",(child1.parent.borrow().upgrade()));
    print!("root 's children:{:#?}",root.get_children());
}
fn main() {

}