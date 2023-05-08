#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "skip")]
    #[test]
    fn test1() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.size, 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[cfg(feature = "skip")]
    #[test]
    fn test2() {
        let mut list = SinglyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.size, 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[cfg(feature = "skip")]
    #[test]
    fn test3() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        list.push_back(2);
        assert_eq!(list.size, 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[cfg(feature = "skip")]
    #[test]
    fn into_iter() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        for item in list {
            println!("item = {item}")
        }
    }
}

fn main() {
    println!("Hello, world!")
}
