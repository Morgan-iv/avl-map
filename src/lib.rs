pub mod iter;
pub mod map;
pub mod tree;

#[cfg(test)]
mod tests {
    use crate::map::AvlMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_iter() {
        let mut map = AvlMap::new();
        map.insert(4, "four");
        map.insert(1, "one");
        map.insert(9, "nine");
        map.insert(2, "two");
        map.insert(6, "six");
        map.insert(7, "seven");
        let v = map.into_iter().collect::<Vec<_>>();
        assert_eq!(
            v,
            [
                (1, "one"),
                (2, "two"),
                (4, "four"),
                (6, "six"),
                (7, "seven"),
                (9, "nine"),
            ]
        );
    }

    #[test]
    fn test_ref_iter() {
        let mut map = AvlMap::new();
        map.insert(4, "four");
        map.insert(1, "one");
        map.insert(9, "nine");
        map.insert(2, "two");
        map.insert(6, "six");
        map.insert(7, "seven");
        let v = map.iter().collect::<Vec<_>>();
        assert_eq!((&1, &"one"), map.iter().next().unwrap());
        assert_eq!(
            v,
            [
                (&1, &"one"),
                (&2, &"two"),
                (&4, &"four"),
                (&6, &"six"),
                (&7, &"seven"),
                (&9, &"nine"),
            ]
        );
    }

    #[test]
    fn test_reverse_iter() {
        let mut map = AvlMap::new();
        map.insert(4, "four");
        map.insert(1, "one");
        map.insert(9, "nine");
        map.insert(2, "two");
        map.insert(6, "six");
        map.insert(7, "seven");
        let v = map.into_iter().rev().collect::<Vec<_>>();
        assert_eq!(
            v,
            [
                (9, "nine"),
                (7, "seven"),
                (6, "six"),
                (4, "four"),
                (2, "two"),
                (1, "one"),
            ]
        );
    }
}
