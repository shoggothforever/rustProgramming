pub struct Counter{
    count:u32
}
impl Counter{
    pub fn new()->Counter{
        Counter{count:0}
    }
}
impl Iterator for Counter{
    type Item=u32;
    fn next(& mut self)->Option<Self::Item>{
        if self.count<5{
            self.count+=1;
            Some(self.count)
        }else{
            None
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
fn counter_test1(){
    let mut c=Counter::new();
        let mut count:u32=0;
       for _ in [1u32..6u32]{
           count+=1;
           if count<5{
               assert_eq!(c.next(),Some(count));
           }else{
               assert_eq!(c.next(),None);
           }
       }

}
    #[test]
    fn counter_test2() {
        let sum:u32=Counter::new()
            .zip(Counter::new().skip(2))
            .map(|(a,b)|a+b)
            .filter(|x|x>&5)
            .sum();
        assert_eq!(sum,14);
    }

}