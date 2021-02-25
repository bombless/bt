
use std::fmt;

#[derive(Clone)]
struct Node(u32, Option<Box<Node>>, Option<Box<Node>>);
type Bitmap = std::collections::HashMap<(u32, u32), char>;

impl Node {
    fn left_width(&self) -> u32 {
        if let Some(ref left) = self.1 {
            return left.width()
        }
        return 0
    }
    fn right_width(&self) -> u32 {
        if let Some(ref right) = self.2 {
            return right.width()
        }
        return 0
    }
    fn self_width(&self) -> u32 {
        let mut ret = 0;
        let mut acc = self.0;
        if self.0 == 0 { return 1 }
        loop {
            if acc == 0 {
                return ret
            }
            acc = acc / 10;
            ret += 1;
        }
    }
    fn width(&self) -> u32 {
        self.left_width() + self.right_width() + self.self_width()
    }
    fn leaf(val: u32) -> Box<Node> {
        Box::new(Node(val, None, None))
    }
    fn some_leaf(val: u32) -> Option<Box<Node>> {
        Some(Self::leaf(val))
    }
    fn left_stick_width(&self) -> u32 {
        if let Some(left) = self.1.as_deref() {
            // println!("{} left stick {} + {}", self.0, left.right_width(), left.self_width());
            return left.right_width() + 1
        }
        return 0
    }
    fn right_stick_width(&self) -> u32 {
        if let Some(right) = self.2.as_deref() {
            return right.left_width() + 1
        }
        return 0
    }

    fn print(&self, padding: u32, height: u32, bitmap: &mut Bitmap) {
        let root_padding = padding + self.left_width();
        // println!("{} root_padding {}", self.0, root_padding);
        let left_stick = self.left_stick_width();
        // println!("{} left stick width {}", self.0, left_stick);
        let right_stick = self.right_stick_width();
        // println!("{} right stick width {}", self.0, right_stick);
        for (idx, c) in self.0.to_string().chars().enumerate() {
            // println!("insert {}", c);
            bitmap.insert((root_padding + idx as u32, height), c);
        }
        if left_stick > 0 {
            let mut x = root_padding - 1;
            let mut y = height + 1;
            for _ in 0 .. left_stick {
                bitmap.insert((x, y), '/');
                x = x - 1;
                y = y + 1;
            }
        }
        let mut x = root_padding + self.self_width();
        let mut y = height + 1;
        for _ in 0 .. right_stick {
            bitmap.insert((x, y), '\\');
            x = x + 1;
            y = y + 1;
        }

        if let Some(left) = self.1.as_deref() {
            // !("printing left");
            left.print(padding, height + left_stick + 1, bitmap);
        }

        if let Some(right) = self.2.as_deref() {
            right.print(root_padding + self.self_width(), height + right_stick + 1, bitmap);
        }

    }
    fn sub_tree(&self) -> Option<Box<Node>> {
        Some(Box::new(self.clone()))
    }

    fn random_acc(mut acc: Node, step: u32, limit: u32) -> (u32, Node) {
        use ::rand::Rng;
        if acc.width() > limit {
            return (step, acc)
        }
        let mut rng = ::rand::thread_rng();
        
        
        let next_step = if rng.gen::<bool>() {
            let (next_step, left) = Self::random_acc(Node(step, None, None), step + 1, limit);
            if left.width() < limit {
                acc.1 = left.sub_tree();
                next_step
            } else {
                step
            }
            
        } else {
            step
        };

        let final_step = if rng.gen::<bool>() {
            let (final_step, right) = Self::random_acc(Node(next_step, None, None), next_step + 1, limit);
            if right.width() < limit {
                acc.2 = right.sub_tree();
                final_step
            } else {
                next_step
            }
        } else {
            next_step
        };
        
        (final_step, acc)
    }

    fn random(lower_limit: u32, higher_limit: u32) -> Node {
        loop {
            let node = Self::random_acc(Node(12345, None, None), 123456, higher_limit).1;
            if node.width() > lower_limit {
                return node
            }
        }
        
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bitmap = Bitmap::new();
        self.print(0, 0, &mut bitmap);
        // println!("{:?}", bitmap);
        let width = *bitmap.iter().map(|((x, _), _)| x).max().unwrap_or(&0);
        let height = *bitmap.iter().map(|((_, y), _)| y).max().unwrap_or(&0);
        for y in 0 ..= height {
            for x in 0 ..= width {
                write!(f, "{}", bitmap.get(&(x, y)).unwrap_or(&' '))?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn main() {
    let tree1 = Node(0, None, None);
    let tree2 = Node(2, Node::some_leaf(1), None);
    let tree3 = Node(3, tree2.sub_tree(), None);
    let tree4 = Node(4, None, tree3.sub_tree());
    let tree5 = Node(233, Some(Box::new(Node(234, None, None))), Some(Box::new(Node(235, None, None))));
    println!("width {}", tree5.width());
    println!("{}", tree5);
    println!("{}", Node::random(30, 40));
}