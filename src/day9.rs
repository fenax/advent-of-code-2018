//5 36
//
//

type Requires = Vec<Vec<u8>>;
// building the prerequisite table
pub fn parse(input:&Vec<&str>)->Requires{
    let mut out = Vec::new();
    out
}


pub fn process()->u32{
    let interval = 23;
    let last = 7105800;
    let player_count = 491 ;

    let mut elves:std::vec::Vec<u32>;
    let mut v = Vec::new();
    let mut turn = 0;
    let mut current;
    elves =
        (0..player_count).map(|_|0 as u32).collect();

    let mut elves_iter
        = (0..elves.len()).cycle();

    let mut next_interval = interval + turn;

    v.push(0);
    current = 0;

    loop{
        let mut elf = elves_iter.next().unwrap();
        turn += 1;
        if turn != next_interval{
            let next_insert =
                  (current + 2) % v.len();
            v.insert(next_insert,turn);
            current = next_insert;
        }else{
            elves[elf] += turn;
            let p 
                = if current >= 7{
                    current - 7
                }else{
                    current + v.len() - 7
                };
            elves[elf] += v[p];
            v.remove(p);
            current = p;
            next_interval += interval;
        }
       // println!("{:?}",v);
        if turn == last {break}

    }
    println!("{:?}", elves);
    *elves.iter().max().unwrap()
}

pub fn process2(input:&Requires)->u32{
    0
}
