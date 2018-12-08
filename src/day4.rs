use std::vec::Vec;
use std::collections::HashMap;
use std::cmp::Ordering;

struct Day{
    guard: i32,
    sleep: Vec<[i32;2]>,
}

#[derive(Eq,Debug)]
struct Sleep{
    total: i32,
    guard: i32,
}

impl Ord for Sleep{
    fn cmp(&self,other: &Sleep)
        -> Ordering{
            self.total.cmp(&other.total)
        }
}
impl PartialOrd for Sleep {
    fn partial_cmp(&self,other: &Sleep)->
    Option<Ordering>{
        self.total.partial_cmp(&other.total)
    }
}
impl PartialEq for Sleep {
    fn eq(&self,other: &Sleep) ->
    bool{
        self.total.eq(&other.total)
    }
}
fn sleep_map(l:&Vec<[i32;2]>)
-> [u8;100]{

    let mut time:[u8;100] =  [0;100];
    for g in l{
        for i in &mut time[(g[0] as usize)..(g[1]as usize)]{
            *i+=1;
        }
    }
    time
}
pub fn process(input:&Vec<[&str;3]>)
-> i32{

    let out = prepare(input);
    let mut sleep: Vec<Sleep> = out.iter().map(
        |(k,v)| {Sleep{guard:*k,total:v.iter()
        .map(|[a,b]| b-a ).sum()}}).collect();
    sleep.sort();
    //println!("  {:?}",sleep);
    let chosen = sleep.last().unwrap().guard;
    
    let time = sleep_map(out.get(&chosen).unwrap());

    let (id, _) = max_index(time.iter());
    
    
    
    (id as i32) * chosen
}

pub fn process2(input:&Vec<[&str;3]>)
-> i32{
    let p = prepare(input);
    let mut psl :Vec<(i32,usize,u8)> 
        = p.iter().map(
        |(id,x)|{
        let (y,z) = 
            max_index(sleep_map(x).iter());
        (*id,y,z)
        }).collect();
    psl.sort_by_key(|k| k.1);
    let last = psl.last().unwrap();
    last.0 * last.1 as i32

}

pub fn max_index<'a,I>(t :I) -> (usize,u8)
where
    I: Iterator<Item= &'a u8 >{

    let mut big:u8 = 0;
    let mut id:usize =0;
    for (i,v) in t.enumerate(){
        if v>&big {big = *v;id = i;}
    }
    (id,big)
}

pub fn prepare(input:&Vec<[&str;3]>)
-> HashMap<i32,Vec<[i32;2]>>{
    let mut out: HashMap<i32,Vec<[i32;2]>> =
        HashMap::new();
//    days = Vec<Day>::new();
    let mut start_sleep = 0;
    let mut guard = 0;
    for s in input{
        let time : i32 = s[0].parse().unwrap();
        match s[1]{
            "#" => {
                guard = 
                    s[2].split_whitespace()
                    .next().unwrap()
                    .parse().unwrap();
            }
            // falls asleep
            "a" => start_sleep = time,
            // wakes up
            "u" => out.entry(guard).or_insert(Vec::new()).push([start_sleep,time]),
            &_ => panic!("aaaaaaaa")
        }
    }
    out
        
}
