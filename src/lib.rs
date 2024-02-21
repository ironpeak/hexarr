pub struct HexArray<T> {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<T>,
}

impl<T> HexArray<T>
where
    T: Clone,
{
    pub fn new(height: usize, width: usize, default: T) -> Self {
        HexArray {
            height,
            width,
            tiles: vec![default; height * width],
        }
    }
}

impl<T> HexArray<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.height && y < self.width {
            Some(&self.tiles[x * self.width + y])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.height && y < self.width {
            Some(&mut self.tiles[x * self.width + y])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
        if x < self.height && y < self.width {
            self.tiles[x * self.width + y] = value;
            Some(())
        } else {
            None
        }
    }

    pub fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x % 2 == 0 {
            if y > 0 {
                result.push((x, y - 1));
                if x < self.width - 1 {
                    result.push((x + 1, y - 1));
                }
                if x > 0 {
                    result.push((x - 1, y - 1));
                }
            }
            if y < self.height - 1 {
                result.push((x, y + 1));
            }
        } else {
            if y < self.height - 1 {
                result.push((x, y + 1));
                if x < self.width - 1 {
                    result.push((x + 1, y + 1));
                }
                if x > 0 {
                    result.push((x - 1, y + 1));
                }
            }
            if y > 0 {
                result.push((x, y - 1));
            }
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if x > 0 {
            result.push((x - 1, y));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_adjacent(left: &Vec<(usize, usize)>, right: &Vec<(usize, usize)>) {
        if left.len() != right.len() {
            assert_eq!(left, right);
        }
        for node in left {
            assert_eq!(right.iter().filter(|x| x == &node).count(), 1);
        }
    }

    #[test]
    fn test_adjacent_0_0() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(&hex_array.adjacent(0, 0), &vec![(0, 1), (1, 0)]);
    }

    #[test]
    fn test_adjacent_0_1() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(0, 1),
            &vec![(0, 0), (0, 2), (1, 0), (1, 1)],
        );
    }

    #[test]
    fn test_adjacent_0_2() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(0, 2),
            &vec![(0, 1), (0, 3), (1, 1), (1, 2)],
        );
    }

    #[test]
    fn test_adjacent_0_3() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(&hex_array.adjacent(0, 3), &vec![(0, 2), (1, 2), (1, 3)]);
    }

    #[test]
    fn test_adjacent_1_0() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(1, 0),
            &vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)],
        );
    }

    #[test]
    fn test_adjacent_1_1() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(1, 1),
            &vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 1), (2, 2)],
        );
    }

    #[test]
    fn test_adjacent_1_2() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(1, 2),
            &vec![(0, 2), (0, 3), (1, 1), (1, 3), (2, 2), (2, 3)],
        );
    }

    #[test]
    fn test_adjacent_1_3() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(&hex_array.adjacent(1, 3), &vec![(0, 3), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_adjacent_2_0() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(&hex_array.adjacent(2, 0), &vec![(1, 0), (2, 1), (3, 0)]);
    }

    #[test]
    fn test_adjacent_2_1() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(2, 1),
            &vec![(1, 0), (1, 1), (2, 0), (2, 2), (3, 0), (3, 1)],
        );
    }

    #[test]
    fn test_adjacent_2_2() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(2, 2),
            &vec![(1, 1), (1, 2), (2, 1), (2, 3), (3, 1), (3, 2)],
        );
    }

    #[test]
    fn test_adjacent_2_3() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(2, 3),
            &vec![(1, 2), (1, 3), (2, 2), (3, 2), (3, 3)],
        );
    }

    #[test]
    fn test_adjacent_3_0() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(&hex_array.adjacent(3, 0), &vec![(2, 0), (2, 1), (3, 1)]);
    }

    #[test]
    fn test_adjacent_3_1() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(3, 1),
            &vec![(2, 1), (2, 2), (3, 0), (3, 2)],
        );
    }

    #[test]
    fn test_adjacent_3_2() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(
            &hex_array.adjacent(3, 2),
            &vec![(2, 2), (2, 3), (3, 1), (3, 3)],
        );
    }

    #[test]
    fn test_adjacent_3_3() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_adjacent(&hex_array.adjacent(3, 3), &vec![(2, 3), (3, 2)]);
    }
}
