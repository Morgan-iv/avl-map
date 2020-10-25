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
    fn test_mut_iter() {
        let mut map = AvlMap::new();
        map.insert(4, "four".to_owned());
        map.insert(1, "one".to_owned());
        map.insert(9, "nine".to_owned());
        map.insert(2, "two".to_owned());
        map.insert(6, "six".to_owned());
        map.insert(7, "seven".to_owned());
        for (_, v) in &mut map {
            v.push_str("_plus");
        }
        let v = map.into_iter().collect::<Vec<_>>();
        assert_eq!(
            v,
            [
                (1, "one_plus".to_owned()),
                (2, "two_plus".to_owned()),
                (4, "four_plus".to_owned()),
                (6, "six_plus".to_owned()),
                (7, "seven_plus".to_owned()),
                (9, "nine_plus".to_owned()),
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

    #[test]
    fn test_get() {
        let mut map = AvlMap::new();
        map.insert(4, "four".to_owned());
        map.insert(1, "one".to_owned());
        map.insert(9, "nine".to_owned());
        assert_eq!(map.get(&1).unwrap(), &"one".to_owned());
    }
    
    #[test]
    fn test_get_mut() {
        let mut map = AvlMap::new();
        map.insert(4, "four".to_owned());
        map.insert(1, "one".to_owned());
        map.insert(9, "nine".to_owned());
        map.get_mut(&4).unwrap().push_str("_plus");
        let v = map.into_iter().collect::<Vec<_>>();
        assert_eq!(
            v,
            [
                (1, "one".to_owned()),
                (4, "four_plus".to_owned()),
                (9, "nine".to_owned()),
            ]
        );
    }
}
