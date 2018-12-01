
pub fn process(input: &String) -> i32{
   input.split('\n').map(|x| x.trim())
                   .map(|x| -> i32 {x.parse().unwrap_or(0)}).sum() 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn day_one_works() {
        assert_eq!(process("+1, +1, +1"), 3);
        assert_eq!(process("+1, +1, -2"), 0);
        assert_eq!(process("-1, -2, -3"),-6);
    }
}
