pub trait Node {
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
