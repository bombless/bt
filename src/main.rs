
use std::fmt;

#[derive(Clone)]
struct NomalTreeNode(u32, Option<Box<Self>>, Option<Box<Self>>);
type Bitmap = std::collections::HashMap<(u32, u32), char>;

impl NomalTreeNode {
    fn sub_tree(&self) -> Option<Box<Self>> {
        Some(Box::new(self.clone()))
    }

    fn some_leaf(value: u32) -> Option<Box<Self>> {
        Some(Box::new(Self::leaf(value)))
    }

    fn random_acc(mut acc: Self, step: u32, limit: u32) -> (u32, Self) {
        use ::rand::Rng;
        if acc.width() > limit {
            return (step, acc)
        }
        let mut rng = ::rand::thread_rng();
        
        
        let next_step = if rng.gen::<u8>() > 200 {
            let (next_step, left) = Self::random_acc(Self::leaf(step), step + 1, limit);
            if left.width() < limit {
                acc.1 = left.sub_tree();
                next_step
            } else {
                step
            }
            
        } else {
            step
        };

        let final_step = if rng.gen::<u8>() > 200 {
            let (final_step, right) = Self::random_acc(Self::leaf(next_step), next_step + 1, limit);
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

    fn random(lower_limit: u32, higher_limit: u32) -> Self {
        loop {
            let node = Self::random_acc(Self::leaf(0), 1, higher_limit).1;
            if node.width() > lower_limit {
                return node
            }
        }
        
    }

    fn print(&self, padding: u32, height: u32, bitmap: &mut Bitmap) {
        let root_padding = padding + self.left_width();
        // println!("{} root_padding {}", self.0, root_padding);
        let left_stick = self.left_stick_width();
        // println!("{} left stick width {}", self.0, left_stick);
        let right_stick = self.right_stick_width();
        // println!("{} right stick width {}", self.0, right_stick);

        let pad = if self.get_value() < 100 { 1 } else { 0 };
        for (idx, c) in self.get_value().to_string().chars().enumerate() {
            // println!("insert {}", c);
            bitmap.insert((root_padding + pad + idx as u32, height), c);
        }
        if left_stick > 0 {
            let mut x = root_padding;
            let mut y = height;
            for _ in 0 .. left_stick {
                x = x - 1;
                y = y + 1;
                bitmap.insert((x, y), '/');
            }
        }
        let mut x = root_padding + self.self_width();
        let mut y = height + 1;
        for _ in 0 .. right_stick {
            bitmap.insert((x, y), '\\');
            x = x + 1;
            y = y + 1;
        }

        if let Some(left) = self.get_left() {
            // !("printing left");
            left.print(padding, height + left_stick + 1, bitmap);
        }

        if let Some(right) = self.get_right() {
            right.print(root_padding + self.self_width(), height + right_stick + 1, bitmap);
        }

    }
}

trait Node {
    fn get_left(&self) -> Option<&Self>;
    fn get_right(&self) -> Option<&Self>;
    fn get_value(&self) -> u32;
    fn leaf(value: u32) -> Self;
    fn set_left(&mut self, left: Option<Self>) where Self: Sized;
    fn set_right(&mut self, right: Option<Self>) where Self: Sized;


    fn left_width(&self) -> u32 {
        if let Some(left) = self.get_left() {
            return left.width()
        }
        return 0
    }
    fn right_width(&self) -> u32 {
        if let Some(right) = self.get_right() {
            return right.width()
        }
        return 0
    }
    fn self_width(&self) -> u32 {
        let mut ret = 0;
        let mut acc = self.get_value();
        if acc == 0 { return 3 }
        loop {
            if acc == 0 {
                return if ret < 3 { 3 } else { ret }
            }
            acc = acc / 10;
            ret += 1;
        }
    }
    fn width(&self) -> u32 {
        self.left_width() + self.right_width() + self.self_width()
    }
    fn left_stick_width(&self) -> u32 {
        if let Some(left) = self.get_left() {
            // println!("{} left stick {} + {}", self.0, left.right_width(), left.self_width());
            return left.right_width() + 1
        }
        return 0
    }
    fn right_stick_width(&self) -> u32 {
        if let Some(right) = self.get_right() {
            return right.left_width() + 1
        }
        return 0
    }
}

impl Node for NomalTreeNode {
    fn get_left(&self) -> Option<&Self> {
        self.1.as_deref()
    }
    fn get_right(&self) -> Option<&Self> {
        self.2.as_deref()
    }
    fn get_value(&self) -> u32 {
        self.0
    }
    fn leaf(val: u32) -> Self {
        NomalTreeNode(val, None, None)
    }
    fn set_left(&mut self, left: Option<Self>) {
        self.1 = left.map(Box::new)
    }
    fn set_right(&mut self, right: Option<Self>) {
        self.2 = right.map(Box::new)
    }
}

impl fmt::Display for NomalTreeNode {
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
    let tree1 = NomalTreeNode(0, None, None);
    let tree2 = NomalTreeNode(2, NomalTreeNode::some_leaf(1), None);
    let tree3 = NomalTreeNode(3, tree2.sub_tree(), None);
    let tree4 = NomalTreeNode(4, None, tree3.sub_tree());
    let tree5 = NomalTreeNode(233, NomalTreeNode::some_leaf(234), NomalTreeNode::some_leaf(235));
    println!("{}", tree1);
    println!("{}", tree4);
    println!("{}", tree5);
    println!("{}", NomalTreeNode::random(30, 40));
}
