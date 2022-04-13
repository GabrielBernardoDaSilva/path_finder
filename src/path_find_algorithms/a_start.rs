// use crate::components::map::{Map, Tile};



#[derive(Clone, Debug)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub g: i32,
    pub h: i32,
    pub f: i32,
    pub parent: Box<Option<Node>>,
}



impl Node {
    pub fn new(x: i32, y: i32, g: i32, h: i32, parent: Box<Option<Node>>) -> Node {
        Node {
            x: x,
            y: y,
            g: g,
            h: h,
            f: g + h,
            parent: parent,
        }
    }

}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn a_start_num(maze: [[u32;5];5], start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> bool{
    let mut open_list: Vec<Node> = Vec::new();
    let mut closed_list: Vec<Node> = Vec::new();

    let mut current_node = Node::new(start_x, start_y, 0, 0, Box::new(None));
    let end_node = Node::new(end_x, end_y, 0, 0, Box::new(None));
    let mut found = false;


    
    
    open_list.push(current_node);

    while open_list.len() > 0 {
        current_node = open_list[0].clone();
        let mut currnet_node_index: u32 = 0;

        for (index, item) in open_list.iter().enumerate() {
            if item.f < current_node.f {
                current_node = item.clone();
                currnet_node_index = index as u32;
            }
        }

        open_list.remove(currnet_node_index as usize);
        closed_list.push(current_node.clone());

        if current_node == end_node {
            let mut current = current_node.clone();
            let mut path: Vec<Node> = Vec::new();
            while current.parent.is_some() {
                path.push(current.clone());
                current = current.parent.unwrap();
            }
            found = true;
            println!("{:?}", found);
            println!("{:?}", path);

            return found;   
        }

        let mut children: Vec<Node> = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_x = current_node.x + i;
                let new_y = current_node.y + j;

                if new_x < 0 || new_x >= maze.len() as i32 || new_y < 0 || new_y >= maze[0].len() as i32 {
                    continue;
                }

                if maze[new_x as usize][new_y as usize] == 1 {
                    continue;
                }

                let new_node = Node::new(new_x, new_y, current_node.g + 1, 0, Box::new(Some(current_node.clone())));
                children.push(new_node);
            }
        }

        for mut child in children {
            let mut in_closed_list: bool = false;
            for closed_child in closed_list.iter() {
                if child == *closed_child {
                    in_closed_list = true;
                    break;
                }
            }

            if in_closed_list {
                continue;
            }

            child.g = current_node.g + 1;
            child.h = (child.x - end_node.x).abs() + (child.y - end_node.y).abs();
            child.f = child.g + child.h;

            let mut in_open_list: bool = false;
            for open_child in open_list.iter() {
                if child == *open_child {
                    in_open_list = true;
                    break;
                }
            }

            if in_open_list {
                continue;
            }

            open_list.push(child.clone());
        }

    }
    return found;
}


// pub fn a_start(map: &Map, start: &Tile, end: &Tile) -> Vec<Node> {


//     unimplemented!();

// }
