#[derive(Debug, PartialEq)]
pub struct Rect {
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

impl Rect {
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
            rect: Rect,
            direction: Direction,
            distance: i32,
            want: Rect,
        }

        let tests = vec![
            TestCase {
                name: "move left 10",
                rect: Rect::new(50, 50, 100, 100),
                direction: Direction::Left,
                distance: 10,
                want: Rect::new(40, 50, 100, 100),
            },
            TestCase {
                name: "move right 10",
                rect: Rect::new(50, 50, 100, 100),
                direction: Direction::Right,
                distance: 10,
                want: Rect::new(60, 50, 100, 100),
            },
        ];

        for test in tests {
            let mut rect = test.rect;
            rect.move_selection(test.direction, test.distance);
            assert_eq!(
                rect, test.want,
                "Test case '{}' failed: want {:?}, got {:?}",
                test.name, test.want, rect
            );
        }
    }
}
