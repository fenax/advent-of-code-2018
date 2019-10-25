//5 36
//
//

type Requires = Vec<Vec<u8>>;
// building the prerequisite table
pub fn parse(input:&Vec<&str>)->Requires{
    let mut out = Vec::new();
    for _i in 0..26{
        out.push(Vec::new());
    }
    for i in input{
        println!(".{}.",i);
        out[i.chars().nth(36).expect("no such char 36") as usize- 'A' as usize ]
            .push(i.chars().nth(5).expect("no such char 5") as u8-'A' as u8);
    }
    out
}


pub fn process(input:&Requires)->String{
    let mut done = [false;26];
    (0..26).map(
        |_|{
            for (i,_) in
                done.iter().enumerate()
                    .filter(|(_,v)|!**v)
            {
                if input[i].iter()
                    .map(|i|done[*i as usize])
                    .all(|v|v)
                {
                    done[i]=true;
                    return (i as u8 +'A' as u8) as char
                }
            }
            panic!("nothing to do");
        }
    ).collect()
}

pub fn process2(input:&Requires)->u32{
    let mut done = [false;26];
    let mut time:u32 = 0;
    // ( task, finish time)
    let mut working:Vec<Option<(u8,u32)>>=[None;5].to_vec();
    loop{
       // give work
       {
       let mut work =
           done.iter().enumerate()
               .filter(|(_,v)| !**v)
               .filter(|(i,_)|{
                   input[*i as usize].iter()
                      .map(|j|done[*j as usize])
                      .all(|v|v)
                })
               .filter(|(i,_)|
                       ! working.iter()
                           .filter_map(|v|*v)
                           .map(|v|v.0)
                           .any(|v| v == *i as u8));
        working =
            working.iter()
                .map(|x|match *x{
                    Some(y) => Some(y),
                    None => match work.next(){
                        Some((i,_)) => 
                            Some((i as u8,(time+60+i as u32+1)as u32)),
                        None => None
                    }
                }).collect()
       }
       //println!("{:?}",&working);
       //freeing up workers.
       let now_work:Vec<(u8,u32)> = 
           working.iter()
               .filter_map(|x|*x)
               .collect();
        let next = now_work.iter()
                       .map(|(i,t)|t)
                       .min();
        match next{
            None => {break;},
            Some(now) =>{
                time = *now;
                working = working.iter()
                    .map(|y|match y{
                        None => None,
                        Some((i,t)) =>
                            if t <= now {
                                done[*i as usize]=true;
                               // println!("finished task {} at {}",(i+'A' as u8)as char,t);
                                None
                            }else{
                                Some((*i,*t))
                            }
                    }).collect();
            }
        }
    }
    time

}
