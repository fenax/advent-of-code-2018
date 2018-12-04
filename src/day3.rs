fn build_grid(input: &Vec<Vec<i32>>) 
-> [[u8;1000];1000]{
    let mut t: [[u8;1000];1000]=[[0;1000];1000];
    for i in input {
        for x in (i[1] as usize)..((i[1]+i[3]) as usize){
            for y in (i[2] as usize)..((i[2]+i[4]) as usize){
                t[x][y]+=1;

            }
        }
    }
t
}

pub fn process(input : &Vec<Vec<i32>>)
-> i32{
    let t = build_grid(input);
    let mut c = 0;
    for i in t.iter(){
        i.iter().for_each(|x| if *x>1 {c+=1;});
    }
    c
}

pub fn process_2(input : &Vec<Vec<i32>>)
-> i32{
    let t = build_grid(input);
    'main: for i in input {
        for x in (i[1] as usize)..((i[1]+i[3]) as usize){
            for y in (i[2] as usize)..((i[2]+i[4]) as usize){
                if t[x][y] != 1 {continue 'main;};

            }
        }
        return i[0];
    }
    9999
}
