use std::vec;

//5 36
//
//
pub struct Star{
    x:i32,
    y:i32,

    x_speed:i32,
    y_speed:i32,
}

type Requires = Vec<Star>;
/*
position=<-10452,  42740> velocity=< 1, -4>
*/

// building the prerequisite table
pub fn parse(input:&Vec<&str>)->Requires{

    input.iter().map(|v|{
        Star{
        x:v[10..16].trim()
            .parse::<i32>().unwrap(),
        y:v[18..24].trim()
            .parse::<i32>().unwrap(),
        x_speed:v[36..38].trim()
            .parse::<i32>().unwrap(),
        y_speed:v[40..42].trim()
            .parse::<i32>().unwrap()
    }}).collect()
}

fn render(input:&Requires, time:i32)->String{
    let positions:Vec<(i32,i32)> = input.iter()
        .map(|i| (i.x + i.x_speed * time,
                  i.y + i.y_speed * time))
        .collect();
    let maxx=positions.iter().map(|x|x.0)
        .max().unwrap() as usize;
    let minx=positions.iter().map(|x|x.0)
        .min().unwrap() as usize;
    let maxy=positions.iter().map(|x|x.1)
        .max().unwrap() as usize;
    let miny=positions.iter().map(|x|x.1)
        .min().unwrap() as usize;

    let line = maxx-minx+2;
    let mut display: vec::Vec<char> = 
        vec![' ';(line*(maxy-miny+1))];

    for y in (0..(maxy-miny+1)){
        
        display[(y+1)*line-1] ='\n';
    }

    for (x,y) in positions{
        let x = x as usize - minx;
        let y = y as usize - miny;
        display[y*line+x] = '#';
    }
    display.into_iter().collect()
}

pub fn process(input:&Requires)->String{
    let x_average:f32 = input.iter()
        .map(|i|i.x as f32 / i.x_speed as f32)
        .map(f32::abs)
        .sum::<f32>() / input.len() as f32;
    let y_average:f32 = input.iter()
        .map(|i|i.y as f32 / i.y_speed as f32)
        .map(f32::abs)
        .sum::<f32>() / input.len() as f32;
    render(input,y_average as i32)
}

pub fn process2(input:&Requires)->u32{
    let y_average:f32 = input.iter()                    .map(|i|i.y as f32 / i.y_speed as f32)          .map(f32::abs)
        .sum::<f32>() / input.len() as f32;

    y_average as u32
}
