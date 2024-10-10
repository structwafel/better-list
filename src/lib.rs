#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkedList<'a, T> {
    Node {
        value: T,
        next: &'a LinkedList<'a, T>,
    },
    Tail,
}

impl<T> Default for LinkedList<'_, T> {
    fn default() -> Self {
        LinkedList::Tail
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prepend(&'a self, value: T) -> Self {
        LinkedList::Node { value, next: self }
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter(self)
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for _ in self.iter() {
            len += 1;
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, LinkedList::Tail)
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        for item in self.iter() {
            if item == value {
                return true;
            }
        }
        false
    }

    pub fn front(&self) -> Option<&T> {
        match self {
            LinkedList::Node { value, .. } => Some(value),
            LinkedList::Tail => None,
        }
    }

    pub fn back(&self) -> Option<&T> {
        match self {
            LinkedList::Node { value, next } => {
                if let LinkedList::Node { value, .. } = next {
                    Some(value)
                } else {
                    Some(value)
                }
            }
            LinkedList::Tail => None,
        }
    }

    pub fn tail(&self) -> Option<&LinkedList<T>> {
        match self {
            LinkedList::Node { next, .. } => Some(next),
            LinkedList::Tail => None,
        }
    }

    pub fn skip(&self, n: usize) -> Option<&LinkedList<T>> {
        match self {
            LinkedList::Node { next, .. } => {
                if n == 0 {
                    Some(self)
                } else {
                    next.skip(n - 1)
                }
            }
            LinkedList::Tail => None,
        }
    }

    pub fn get(&self, n: usize) -> Option<&T> {
        self.skip(n).and_then(LinkedList::front)
    }
}

#[macro_export]
macro_rules! list {
    () => {
        LinkedList::Tail
    };
    ($first:expr $(, $rest:expr)*) => {
        LinkedList::Node {
            value: $first,
            next: &list!($($rest),*)
        }
    };
}

/// trying to make a macro which can be used with vec and array variables
// #[macro_export]
// macro_rules! list_from {
//     // Case for a vector literal
//     (vec![$($elem:expr),*]) => {
//         list!($($elem),*)
//     };
//     // Case for an array
//     ([$($elem:expr),*]) => {
//         list!($($elem),*)
//     };
//     // Case for separate items
//     ($($elem:expr),*) => {
//         list!($($elem),*)
//     };
// }

#[test]
fn macro_test() {
    let list = list![1, 2, 3, 4, 5];

    assert_eq!(list.len(), 5);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.contains(&3), true);
    assert_eq!(list.contains(&6), false);
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&5));
    assert_eq!(list.get(2), Some(&3));
    assert_eq!(list.get(10), None);
    assert_eq!(list.tail(), Some(&list![2, 3, 4, 5]));
    assert_eq!(list.skip(2), Some(&list![3, 4, 5]));
    assert_eq!(list.skip(10), None);
    assert_eq!(list.skip(10).is_none(), true);

    fn check_for_len_6(ll: LinkedList<u32>) {
        assert_eq!(ll.len(), 6);
    }

    check_for_len_6(list.prepend(3));
}

pub struct ListIter<'a, T>(&'a LinkedList<'a, T>);

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            LinkedList::Node { value, next } => {
                println!("{}", 1);
                self.0 = next;
                Some(value)
            }
            LinkedList::Tail => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<'a, T> {
    type Item = &'a T;

    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}
