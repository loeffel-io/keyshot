#[derive(Debug, PartialEq)]
pub struct SelectionRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl SelectionRect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn move_selection(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Left => self.x -= distance,
            Direction::Right => self.x += distance,
            Direction::Up => self.y -= distance,
            Direction::Down => self.y += distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_selection_left() {
        struct TestCase {
            name: &'static str,
            selection_rect: SelectionRect,
            direction: Direction,
            distance: i32,
            want: SelectionRect,
        }

        let tests = vec![
            TestCase {
                name: "move left 10",
                selection_rect: SelectionRect::new(50, 50, 100, 100),
                direction: Direction::Left,
                distance: 10,
                want: SelectionRect::new(40, 50, 100, 100),
            },
            TestCase {
                name: "move right 10",
                selection_rect: SelectionRect::new(50, 50, 100, 100),
                direction: Direction::Right,
                distance: 10,
                want: SelectionRect::new(60, 50, 100, 100),
            },
        ];

        for test in tests {
            let mut selection_rect = test.selection_rect;
            selection_rect.move_selection(test.direction, test.distance);
            assert_eq!(
                selection_rect, test.want,
                "Test case '{}' failed: want {:?}, got {:?}",
                test.name, test.want, selection_rect
            );
        }
    }
}
