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
    let player_count = 491;

    let mut elves:std::vec::Vec<u32>;
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut turn = 0;
    let mut current;
    elves =
        (0..player_count).map(|_|0 as u32).collect();

    let mut elves_iter
        = (0..elves.len()).cycle();

    let mut next_interval = interval + turn;

    left.push(0);
    current = 0;

    loop{
        let elf = elves_iter.next().unwrap();
        let len = left.len() + right.len();

        turn += 1;

        if turn != next_interval{
            if right.is_empty(){
                let mut tmp = left;
                tmp.reverse();
                
                left = right;
                right = tmp;
            }
            left.push(right.pop().unwrap());
            left.push(turn);
            current =
                  (current + 2) % len;
        }else{
            elves[elf] += turn;
            if left.len() > 7{
                for _ in (0..7){
                    right.push(
                        left.pop().unwrap());
                }
                elves[elf] 
                    += left.pop().unwrap();
            }else{
                let rest = left.len();
                for _ in (0..rest){
                    right.push(
                        left.pop().unwrap());
                }
                {
                    let mut tmp = right;
                    tmp.reverse();
                    right = left;
                    left = tmp;
                }
                for _ in (rest..7){
                    right.push(
                        left.pop().unwrap());
                }
                elves[elf]
                    += left.pop().unwrap();
            }
            left.push(right.pop().unwrap());
            current
                = if current >= 7{
                    current - 7
                }else{
                    current + len - 7
                };
            next_interval += interval;
        }
      // println!("{:?}{:?}",left,right);
        if turn == last {break}

    }
    println!("{:?}", elves);
    *elves.iter().max().unwrap()
}

pub fn process2(input:&Requires)->u32{
    0
}
