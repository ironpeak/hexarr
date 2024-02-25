#[cfg(feature = "serde")]
mod serde;

pub struct HexArray<T> {
    height: usize,
    width: usize,
    tiles: Vec<T>,
}

impl<T> HexArray<T>
where
    T: Clone,
{
    /// Create a new HexArray with the given height, width, and default value.
    pub fn new(height: usize, width: usize, default: T) -> Self {
        HexArray {
            height,
            width,
            tiles: vec![default; height * width],
        }
    }
}

impl<T> HexArray<T> {
    /// Get the height of the HexArray.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the width of the HexArray.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the value at the given indices.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.height && y < self.width {
            Some(&self.tiles[x * self.width + y])
        } else {
            None
        }
    }

    /// Get a mutable reference to the value at the given indices.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.height && y < self.width {
            Some(&mut self.tiles[x * self.width + y])
        } else {
            None
        }
    }

    /// Set the value at the given indices.
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
        if x < self.height && y < self.width {
            self.tiles[x * self.width + y] = value;
            Some(())
        } else {
            None
        }
    }

    fn pos(x: usize, y: usize) -> (f32, f32) {
        const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
        const HALF_SQRT_3: f32 = 0.8660254037844386467637231707529361835_f32;
        if x % 2 == 0 {
            let x = x as f32 * 1.5;
            let y = y as f32 * SQRT_3;
            (x, y)
        } else {
            let x = x as f32 * 1.5;
            let y = y as f32 * SQRT_3 + HALF_SQRT_3;
            (x, y)
        }
    }

    #[cfg(feature = "glam")]
    /// Get the position of the tile at the given indices.
    pub fn position(x: usize, y: usize) -> glam::Vec2 {
        let (x, y) = Self::pos(x, y);
        glam::Vec2::new(x, y)
    }

    #[cfg(not(feature = "glam"))]
    /// Get the position of the tile at the given indices.
    pub fn position(x: usize, y: usize) -> (f32, f32) {
        Self::pos(x, y)
    }

    /// Get the indices of the tiles adjacent to the given indices.
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

    // new tests

    #[test]
    fn test_new() {
        let hex_array = HexArray::new(4, 4, 0);
        assert_eq!(hex_array.height(), 4);
        assert_eq!(hex_array.width(), 4);
        for x in 0..4 {
            for y in 0..4 {
                assert_eq!(hex_array.get(x, y), Some(&0));
            }
        }
    }

    // get tests

    #[test]
    fn test_get() {
        let mut hex_array = HexArray::new(4, 4, 0);
        hex_array.set(1, 1, 1);
        assert_eq!(hex_array.get(1, 1), Some(&1));
        assert_eq!(hex_array.get(1, 2), Some(&0));
        assert_eq!(hex_array.get(2, 1), Some(&0));
        assert_eq!(hex_array.get(2, 2), Some(&0));
        assert_eq!(hex_array.get(4, 4), None);
    }

    // get_mut tests

    #[test]
    fn test_get_mut() {
        let mut hex_array = HexArray::new(4, 4, 0);
        *hex_array.get_mut(1, 1).unwrap() = 1;
        assert_eq!(hex_array.get(1, 1), Some(&1));
        assert_eq!(hex_array.get(1, 2), Some(&0));
        assert_eq!(hex_array.get(2, 1), Some(&0));
        assert_eq!(hex_array.get(2, 2), Some(&0));
        assert_eq!(hex_array.get(4, 4), None);
    }

    // set tests

    #[test]
    fn test_set() {
        let mut hex_array = HexArray::new(4, 4, 0);
        hex_array.set(1, 1, 1);
        assert_eq!(hex_array.get(1, 1), Some(&1));
        assert_eq!(hex_array.get(1, 2), Some(&0));
        assert_eq!(hex_array.get(2, 1), Some(&0));
        assert_eq!(hex_array.get(2, 2), Some(&0));
        assert_eq!(hex_array.get(4, 4), None);
    }

    // position tests

    #[test]
    fn test_position_0_0() {
        assert_eq!(HexArray::<i32>::position(0, 0), (0.0, 0.0));
    }

    #[test]
    fn test_position_0_1() {
        assert_eq!(HexArray::<i32>::position(0, 1), (0.0, 1.7320508));
    }

    #[test]
    fn test_position_1_0() {
        assert_eq!(HexArray::<i32>::position(1, 0), (1.5, 0.8660254));
    }

    #[test]
    fn test_position_1_1() {
        assert_eq!(HexArray::<i32>::position(1, 1), (1.5, 2.598076));
    }

    // adjacent tests

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
