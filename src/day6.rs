use std::vec::Vec;

struct Map<T>{
    data: Vec<T>,
    outside: T,
    x_size: isize,
    y_size: isize,
}

impl Index for Map<T>{
    type Output = &T;
    fn index(&self,[x:isize,y:isize])->&T{
        if x<0 || y <0 || x>= self.x_size || y>=self.y_size {return T::default()}
        self.data[x+y*self.x_size]
    }
}

impl IndexMut for Map<T>{
    fn index_mut(&mut self,[x:isize,y:isize]) -> &mut T{
        assert!(x >= 0);
        assert!(y >= 0);
        assert!(x < self.x_size);
        assert!(y < self.y_size);
        &mut self.data[x+y*self.x_size]
    }
}

impl Map<T:Clone+Default>{
    fn new(x:isize,y:isize) 
    -> Map{
        Map{
            data: vec![T::default();x_size*y_size],
            outside: T::default(),
            x_size: x,
            y_size: y,
        }
    }
    fn calculate_new(src:Map<T>,
     fn proc(ct:&T,lt:&T,up:&T,rg:&T,dn:&T)->T)
    {
        let mut out = 
Map<T>::new(src.x_size,src.y_size);
        for x in 0..src.x_size{
            for y in 0..src.y_size{
            out[[x,y]] = proc(
               src[[x,y]],
               src[[x-1,y]],src[[x,y-1]],
               src[[x+1,y]],src[[x,y+1]]);
            }
        }
    }
}

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
            Spot::Zero => *self = other,
            Spot::One(x) =>{
                match other{
                    Spot::More =>{
                        *self = Spot::More;
                    },
                    Spot::One(y) =>{
                        if x!=y {
                            *self=Spot::More;
                        }
                    }
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
                let mut s = Spot::None;
                s.set(left);
                s.set(up);
                s.set(right);
                s.set(down);
                s
            }
            x => x
        }
    }
}

process(input: &Vec<(i32,i32)>)(

    )
