use std::str::Bytes;
use std::cmp::max;


pub fn other_process(input: &String)->i32{
    let mut s = react(input);

    while s.1 >0{
        s = react(&s.0);
    }
    s.0.chars().count() as i32
}

pub fn process(input: &String)->i32{
    let mut s = input.as_bytes().to_vec();
    inprocess(s)
}
fn inprocess(mut s: Vec<u8>) -> i32 {
    react2(&mut s);
    let o :Vec<u8> =
        s.iter().filter_map(|x|  if *x != '0' as u8{ Some(*x)}else{None}).collect();
    o.len() as i32
}

pub fn process2(input: &String) -> i32{
    let mut s = input.as_bytes().to_vec();
    (('A' as u8) ..= ('Z' as u8)).map(
        |l| {inprocess((s.iter().filter_map(
            |x| {if *x == l || *x == l+32 {
                None
            }else{
                Some(*x)
            }}).collect::<Vec<u8>>()))})
    .min().unwrap()
}

fn react2 (s:  &mut Vec<u8>){
    fn try_remove(s: &mut Vec<u8>,
                  left: &mut usize,
                  right:usize)
    {
        for i in (0..=*left).rev(){
            if s[i] != '0' as u8{
                *left = i;
                break;
            }
        }
        if s[*left] != s[right]
            && s[*left].eq_ignore_ascii_case(&s[right]){
                s[*left] = '0' as u8;
                s[right] = '0' as u8;
            }else{
                *left = right;
            }
    }
    let mut left = 0 as usize;
    for i in (1..s.len()){
        try_remove(s,&mut left, i);
    }
}


fn react (s: &String)
    -> (String,i32){
        let mut skip=false;
        let mut out = String::new();
        let mut c = 0;
        for (i,j) in s.chars().zip(s.chars().skip(1).chain("0".chars())){
            if skip { 
                skip = false;
                continue;
            }
            if i != j && i.eq_ignore_ascii_case(&j) {
            skip = true;
            c += 1;
            }else{
                out.push(i);
            }

        }
        (out,c)
    }

#[cfg(test)]
mod tests{
    #[test]
    fn test_day5(){
        assert_eq!(::day5::process(&"dabAcCaCBAcCcaDA".to_string()),10);
        let mut v =  "GbbBBaAghAf".as_bytes().to_vec();
        ::day5::react2(&mut v);
        assert_eq!(v,"00000000hAf".as_bytes().to_vec());
        let mut v =  "gftgvbBMmVyYGtfg".as_bytes().to_vec();
        ::day5::react2(&mut v);
        assert_eq!(v,"gft0000000000tfg".as_bytes().to_vec());
        let mut v =  "UmMtvvVgGThHu".as_bytes().to_vec();
        ::day5::react2(&mut v);
        assert_eq!(v,"U00tv0000T00u".as_bytes().to_vec());
    }
}
