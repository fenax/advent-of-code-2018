use std::vec::Vec;
use std::cmp;
use std::ops::{Index,IndexMut};
use std::f32;

struct Map<T>{
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
impl Map<Spot>
{
    fn calculate_new(src:Map<Spot>)
        ->(Map<Spot>,u32){

        let mut out : Map<Spot> = 
            Map::new(src.x_size,src.y_size);
        let mut count = 0;
        for x in 0..src.x_size{
            for y in 0..src.y_size{
            out[(x,y)] = Spot::new(
               &src[(x,y)],
               &src[(x-1,y)],&src[(x,y-1)],
               &src[(x+1,y)],&src[(x,y+1)]);
            if out[(x,y)] != src[(x,y)]{
                count += 1;
            }
            }
        }
        (out,count)
    }
}
#[derive(Clone,PartialEq)]
enum Spot{
    Zero,
    One(i16),
    More,
}

impl Default for Spot{
    fn default()->Spot{
        Spot::Zero
    }
}

impl Spot{
    fn set(&mut self,other:&Spot){
        if let Spot::Zero=other{return}
        match self{
            Spot::Zero => *self = other.clone(),
            Spot::One(x) =>{
                match other{
                    Spot::More =>{
                        *self = Spot::More;
                    },
                    Spot::One(y) =>{
                        if x!=y {
                            *self=Spot::More;
                        }
                    },
                    _ => panic!("impossible")
                }
            },
            _ => {},       
        }
    }
    fn new(center: &Spot,
           left: &Spot, up: &Spot, 
           right: &Spot, down: &Spot)
    ->Spot{
        match center{
            Spot::Zero =>{
                let mut s = Spot::Zero;
                s.set(left);
                s.set(up);
                s.set(right);
                s.set(down);
                s
            }
            x => x.clone()
        }
    }
}

fn prepare(input: &String)->Vec<(isize,isize)>{
   let mut ivec: Vec<&str> = input.split('\n').filter(|s| s.len()>0).collect(); 
   ivec.iter().map(|s|{let mut v= s.split(',');(v.next().unwrap().trim().parse::<isize>().expect("x"),v.next().unwrap().trim().parse::<isize>().expect("y"))}).collect()
}

fn get_size(input:&Vec<(isize,isize)>)->(isize,isize){
let mut max_x = 0;                              let mut max_y = 0;                              input.iter().for_each(|(x,y)|{ max_x = cmp::
max(max_x,*x); max_y = cmp::max(max_y,*y);});
(max_x+1,max_y+1)
}

pub fn process2(input: &String)->u32{
    let mut input = prepare(&input);
    let (max_x,max_y) = get_size(&input);
    let mut grid : Map<u32> = Map::new(max_x,max_y);
    let mut count = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            grid[(x,y)] =
                input.iter().map(|(x2,y2)|{
                    ((x as i32- *x2 as i32).abs()
                    + (y as i32 - *y2 as i32).abs()) as u32
                } ).sum();
            if grid[(x,y)] < 10000{
                count +=1;
            }
        }
    } 
    count
}
pub fn process(input: &String)->i16{
    let mut input = prepare(&input);
    let (max_x,max_y) = get_size(&input);
    let mut grid : Map<Spot> = Map::new(max_x,max_y);
    let size = input.len();
    println!("{},x{}y{}",size,max_x,max_y);
    for (i,p) in input.iter().enumerate(){
        println!("{}",i);
        grid[*p] = Spot::One(i as i16);
    }
    let grid =
        loop{
        let (g,c) = Map::calculate_new(grid);
        if c > 0 {
            grid = g;
        }else{
            break g;
        }
    };
    let mut count = vec![0.0;size];
    let mut set = |x,y,v| {
        let g = grid[(x as isize,y as isize)].clone();
        match g{
            Spot::One(x) =>
                count[x as usize] = v,
            _ => {},
        }
    };
    for i in 0..max_x{
        set(i,0,f32::INFINITY);
        set(i,max_y-1,f32::INFINITY);
    }
    for i in 1..(max_y-1){
        set(0,i,f32::INFINITY);
        set(max_x-1,i,f32::INFINITY);
    }

    let mut inc = |x,y| {
        let g = grid[(x as isize,y as isize)].clone();
        match g {
            Spot::One(x) =>
                count[x as usize] += 1.0,
            _ => {},
        }
    };
    for i in 1..(max_y-1){
        for j in 1..(max_x-1){
            inc(j,i);
        }
    }
    let mut idx:Vec<usize> = (0..size).filter(|v| count[*v].is_finite()).collect();
    idx.sort_by(|a,b|count[*a].partial_cmp(&count[*b]).unwrap());
    count[idx[idx.len()-1]] as i16
}
