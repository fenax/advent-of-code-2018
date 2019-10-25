//5 36
//
//

pub struct Node{
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node{
    fn sum_children(&self)->u32{
        self.children.iter()
            .map(|x|x.sum()).sum::<u32>()
    }
    fn sum(&self)->u32{
        self.sum_children()
        + 
        self.metadata.iter()
            .sum::<u32>()
    }
    fn value(&self)->u32{
        let c = self.children.len() as u32;
        if c == 0 {
            self.metadata.iter()
                .sum::<u32>()
        }else{
            self.metadata.iter()
                .filter_map(|x|
                    if *x > 0 && *x <= c{
                        Some(self.children[*x as usize -1]
                             .value())
                    }else{
                        None
                    }).sum()
        }
    }

    fn parse(input:&mut impl Iterator<Item =  u32>)->Node
        {
        let children_count =
            input.next().unwrap();
        let metadata_count =
            input.next().unwrap();
        let children =
            (0..children_count)
            .map(|_|Node::parse(input))
            .collect();
        let metadata =
            (0..metadata_count)
            .map(|_|input.next().unwrap())
            .collect();
        Node{
            children,
            metadata,
        }
    }
}


type Requires = Node;
// building the prerequisite table
pub fn parse(input:&Vec<&str>)->Requires{
    let mut iterat =
        input.iter()
        .map(|x|
                x.trim()
                 .parse::<u32>()
                 .expect("failed to int"));
    Node::parse(&mut iterat)

}


pub fn process(input:&Requires)->u32{
    input.sum()
}

pub fn process2(input:&Requires)->u32{
    input.value()
}
