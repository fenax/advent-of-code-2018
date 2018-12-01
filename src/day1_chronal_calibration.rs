use std::vec::*;


pub fn process(input: &Vec<i32>) -> i32{
   input.iter().sum() 
}

pub fn process2(input: &Vec<i32>) -> i32{
   let mut tot = 0;
   let mut store: Vec<i32> = Vec::new();
   for x in input.iter().cycle()
   {
       store.push(tot);
       tot+=x; 
       //println!(" {}",tot);
       match store.iter().find(|&&x| tot == x) {
           Some(x) => {eprintln!(" took {} steps", store.len()); return *x},
           None    => continue,
       }
   };
   panic!("did not find anything {:?}",store);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn day_one_works() {
        assert_eq!(process(&vec![ 1, 1, 1]), 3);
        assert_eq!(process(&vec![ 1,  1, -2]), 0);
        assert_eq!(process(&vec![-1, -2, -3]),-6);
    }
    #[test]
    fn day_one_1_works() {
        assert_eq!(process2(&vec![1, -1]),0);
        assert_eq!(process2(&vec![3, 3, 4, -2, -4]),10);
        assert_eq!(process2(&vec![-6, 3,8, 5, -6]),5);
        assert_eq!(process2(&vec![7, 7, -2, -7, -4]),14);
    }
}
