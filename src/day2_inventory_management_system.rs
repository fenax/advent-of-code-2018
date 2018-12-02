use std::vec::*;

struct Cnt (bool,bool);

fn count(input: &Vec<&str>) 
-> Vec<Cnt>{
    input.iter().map(
        |s| {
        let mut old_c = '.';
        let mut count = 0;
        let mut dub = false;
        let mut tri = false;
        let mut v: Vec<char> = s.chars().collect();
        v.sort_unstable();
        v.push('.');
        v.iter().for_each(
            |c| {
                if *c != old_c {
                if count == 2 {dub = true;};
                 if count == 3 {tri = true;};
                 count = 0;};
             old_c = *c;
             count+=1;
        });
        Cnt(dub,tri)}).collect()

}

pub fn different_chars(a: &str,b: &str) -> i32 {
    let mut count = 0;
    a.chars().zip(b.chars()).for_each(|c| if c.0 != c.1 {count+=1;});
    count
}

pub fn process2(input: &Vec<&str>) -> String {
    for (i,a) in input.iter().enumerate(){
        for b in input[(i)..].iter(){
            if different_chars(a,b) == 1 {
                return
                a.chars().zip(b.chars())
                    .filter_map(
                    |c| {
                    if c.0 == c.1 {
                        Some(c.0)
                    }else{
                        None
                    }}).collect()
            }
        }
    }
    "***".to_string()
}

pub fn process(input: &Vec<&str>) -> i32 {
    let mut tri = 0;
    let mut dub = 0;
    count(input).iter().for_each(
        |x|{
            if x.0 {dub+=1;};
            if x.1 {tri+=1;};
        });
    tri * dub
}

#[cfg(test)]
mod tests{

    #[test]
    fn day2_1_works(){
        assert_eq!(::day2_inventory_management_system::process(&vec!("abcdef",
        "bababc","abbcde","abcccd","aabcdd",
        "abcdee","ababab")), 12);
    }
    #[test]
    fn day2_2_works(){ 
        assert_eq!(::day2_inventory_management_system::process2(
                &vec!("abcde","fghij","klmno","pqrst","fguij","axcye","wvxyz")),"fgij");
    }
}
