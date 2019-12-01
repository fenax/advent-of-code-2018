use std::vec::Vec;
use std::cmp;
use std::ops::{Index,IndexMut};
use std::f32;


#[derive(Clone)]
pub struct Map<T>{
    data: Vec<T>,
    outside: T,
    x_size: isize,
    y_size: isize,
}

impl<T> Index<(isize,isize)> for Map<T>
where T : Clone + Default{
    type Output = T;
    fn index(&self,(x,y):(isize,isize))
    ->&Self::Output
    {
        if x<0 || y <0 || x>= self.x_size || y>=self.y_size {return &self.outside}
        &self.data[(x+y*self.x_size)as usize]
    }
}

impl<T> IndexMut<(isize,isize)> for Map<T>
    where T : Clone + Default
{
    //type Output = T;
    fn index_mut(&mut self,(x,y):(isize,isize))     -> &mut T
    {
        assert!(x >= 0);
        assert!(y >= 0);
        assert!(x < self.x_size);
        assert!(y < self.y_size);
        &mut self.data[(x+y*self.x_size) as usize]
    }
}

impl<T> Map<T>
    where T : Clone + Default
{
    fn new(x:isize,y:isize) 
    -> Map<T>{
        Map{
            data: vec![T::default();(x*y)as usize],
            outside: T::default(),
            x_size: x,
            y_size: y,
        }
    }
}
//
//

type Requires = Map<i32>;
// building the prerequisite table
pub fn parse(input:i32)->Requires{
   let mut out :Map<i32> = Map::new(300,300);
   for y in (0..300){
       for x in (0..300){
           let gx = x+1;
           let gy = y+1;
           let rack_id = gx+10;
           let power = rack_id * gy + input;
           let power = power * rack_id;
           let digit = (power / 100) % 10;
           print!("{} ",digit);
           out[(x as isize,y as isize)] 
               = digit - 5;
       }
       println!("");
   }
   out
}

pub fn process(input:&Requires)->(i32,i32){
   // let total : Map<i32> = Map::new(300,300);
    let mut largest:(i32,i32) = (0,0);
    let mut value: i32 = -50;

    for y in (1..299){
        for x in (1..299){
            let current =
input[(x-1,y-1)]+input[(x-1,y)]+input[(x-1,y+1)]+
input[(x+0,y-1)]+input[(x+0,y)]+input[(x+0,y+1)]+
input[(x+1,y-1)]+input[(x+1,y)]+input[(x+1,y+1)];
            if current > value{
                value = current;
                largest = (x as i32,y as i32);
            }
        }
    }
    largest
    
}

pub fn process2(input:&Requires)->(i32,i32,i32){
    let mut intermediate:Map<i32>=input.clone();
   let mut largest :(i32,i32,i32) = (0,0,0);
   let mut value: i32 = -50;

   for s in (1..=300){
       for y in (0..=300-s){
           for x in (0..=300-s){
               let mut current = 0;
               let mut count = 0;
               for ix in(0..s){
                       count += 1;
                       current 
                           +=intermediate
                             [(x+ix,y)];
               }
               if current > value{
                   value = current;
                   largest = (x as i32+1,
                              y as i32 +1,
                              s as i32);
               }
               intermediate[(x,y)]+=
                   input[(x,y+s)];
           }

       }
   }
   largest
}
