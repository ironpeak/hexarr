#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HexArray<T> {
    height: usize,
    width: usize,
    tiles: Vec<T>,
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
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

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

// #[cfg(feature = "serde")]
// impl<T> serde::Serialize for HexArray<T>
// where
//     T: serde::Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("HexArray", 3)?;
//         state.serialize_field("height", &self.height)?;
//         state.serialize_field("width", &self.width)?;
//         state.serialize_field("tiles", &self.tiles)?;
//         state.end()
//     }
// }

// #[cfg(feature = "serde")]
// impl<T> serde::Deserialize for HexArray<T>
// where
//     T: serde::Deserialize,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer,
//     {
//         #[derive(serde::Deserialize)]
//         #[serde(field_identifier, rename_all = "lowercase")]
//         enum Field {
//             Height,
//             Width,
//             Tiles,
//         }

//         struct HexArrayVisitor<T> {
//             marker: std::marker::PhantomData<T>,
//         }

//         impl<T> serde::de::Visitor for HexArrayVisitor<T>
//         where
//             T: serde::Deserialize,
//         {
//             type Value = HexArray<T>;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("struct HexArray")
//             }

//             fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess,
//             {
//                 let mut height = None;
//                 let mut width = None;
//                 let mut tiles = None;
//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         Field::Height => {
//                             if height.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("height"));
//                             }
//                             height = Some(map.next_value()?);
//                         }
//                         Field::Width => {
//                             if width.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("width"));
//                             }
//                             width = Some(map.next_value()?);
//                         }
//                         Field::Tiles => {
//                             if tiles.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("tiles"));
//                             }
//                             tiles = Some(map.next_value()?);
//                         }
//                     }
//                 }
//                 let height = height.ok_or_else(|| serde::de::Error::missing_field("height"))?;
//                 let width = width.ok_or_else(|| serde::de::Error::missing_field("width"))?;
//                 let tiles = tiles.ok_or_else(|| serde::de::Error::missing_field("tiles"))?;
//                 Ok(HexArray {
//                     height,
//                     width,
//                     tiles,
//                 })
//             }
//         }

//         const FIELDS: &[&str] = &["height", "width", "tiles"];
//         deserializer.deserialize_struct("HexArray", FIELDS, HexArrayVisitor {
//             marker: std::marker::PhantomData,
//         })
//     }
// }

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
