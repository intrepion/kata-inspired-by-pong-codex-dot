// Copied from glib 0.18.5 src/variant_iter.rs tests, with only crate -> glib import.
// Upstream MIT license and copyright are retained in vendor/glib-0.18.5.
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use glib::{
        prelude::*,
        variant::{DictEntry, Variant},
    };

    #[test]
    fn test_variant_iter_variant() {
        let v = Variant::from_variant(&"foo".to_string().to_variant());
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string()]);
    }

    #[test]
    fn test_variant_iter_array() {
        let v = Variant::array_from_iter::<String>([
            "foo".to_string().to_variant(),
            "bar".to_string().to_variant(),
        ]);
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        let a = vec!["foo".to_string(), "bar".to_string()];
        assert_eq!(&vec, &a);
        let vec: Vec<_> = v.array_iter_str().unwrap().collect();
        assert_eq!(&vec, &a);
    }

    #[test]
    fn test_variant_iter_tuple() {
        let v = Variant::tuple_from_iter([
            "foo".to_string().to_variant(),
            "bar".to_string().to_variant(),
        ]);
        let vec: Vec<String> = v.iter().map(|i| i.get().unwrap()).collect();
        assert_eq!(vec, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_variant_iter_dictentry() {
        let v = DictEntry::new("foo", 1337).to_variant();
        println!("{:?}", v.iter().collect::<Vec<_>>());
        assert_eq!(v.iter().count(), 2);
    }

    #[test]
    fn test_variant_iter_map() {
        let mut map = HashMap::new();
        map.insert("foo", 1);
        map.insert("bar", 1);
        let v = map.to_variant();
        assert_eq!(v.iter().count(), 2);
    }

    #[test]
    fn test_variant_iter_nth() {
        let v = Variant::array_from_iter::<String>([
            "0".to_string().to_variant(),
            "1".to_string().to_variant(),
            "2".to_string().to_variant(),
            "3".to_string().to_variant(),
            "4".to_string().to_variant(),
            "5".to_string().to_variant(),
        ]);

        let mut iter = v.iter();

        assert_eq!(iter.len(), 6);
        assert_eq!(
            iter.nth(1).map(|v| v.get::<String>().unwrap()),
            Some("1".into())
        );
        assert_eq!(iter.len(), 4);
        assert_eq!(
            iter.next().map(|v| v.get::<String>().unwrap()),
            Some("2".into())
        );
        assert_eq!(
            iter.nth_back(2).map(|v| v.get::<String>().unwrap()),
            Some("3".into())
        );
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_variant_iter_count() {
        let v = Variant::array_from_iter::<String>([
            "0".to_string().to_variant(),
            "1".to_string().to_variant(),
            "2".to_string().to_variant(),
        ]);

        let iter = v.iter();

        assert_eq!(iter.len(), 3);
        assert_eq!(iter.count(), 3);
    }

    #[test]
    fn test_variant_iter_last() {
        let v = Variant::array_from_iter::<String>([
            "0".to_string().to_variant(),
            "1".to_string().to_variant(),
            "2".to_string().to_variant(),
        ]);

        let iter = v.iter();

        assert_eq!(iter.len(), 3);
        assert_eq!(
            iter.last().map(|v| v.get::<String>().unwrap()),
            Some("2".into())
        );
    }

    #[test]
    fn test_variant_str_iter_nth() {
        let v = Variant::array_from_iter::<String>([
            "0".to_string().to_variant(),
            "1".to_string().to_variant(),
            "2".to_string().to_variant(),
            "3".to_string().to_variant(),
            "4".to_string().to_variant(),
            "5".to_string().to_variant(),
        ]);

        let mut iter = v.array_iter_str().unwrap();

        assert_eq!(iter.len(), 6);
        assert_eq!(iter.nth(1), Some("1"));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some("2"));
        assert_eq!(iter.nth_back(2), Some("3"));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_variant_str_iter_count() {
        let v = Variant::array_from_iter::<String>([
            "0".to_string().to_variant(),
            "1".to_string().to_variant(),
            "2".to_string().to_variant(),
        ]);

        let iter = v.array_iter_str().unwrap();

        assert_eq!(iter.len(), 3);
        assert_eq!(iter.count(), 3);
    }

    #[test]
    fn test_variant_str_iter_last() {
        let v = Variant::array_from_iter::<String>([
            "0".to_string().to_variant(),
            "1".to_string().to_variant(),
            "2".to_string().to_variant(),
        ]);

        let iter = v.array_iter_str().unwrap();

        assert_eq!(iter.len(), 3);
        assert_eq!(iter.last(), Some("2"));
    }
}
