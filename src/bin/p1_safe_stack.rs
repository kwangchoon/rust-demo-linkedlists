#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "skip")]
    #[test]
    fn basics() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.size, 2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[cfg(feature = "skip")]
    #[test]
    fn peek() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));

        assert_eq!(stack.peek_mut(), Some(&mut 1));
    }

    #[cfg(feature = "skip")]
    #[test]
    fn into_iter() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);

        for item in stack {
            println!("item = {}", item)
        }
    }

    #[cfg(feature = "skip")]
    #[test]
    fn iter() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);

        for item in stack.iter() {
            println!("item = {}", item)
        }
    }

    #[cfg(feature = "skip")]
    #[test]
    fn iter_mut() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);

        for item in stack.iter_mut() {
            *item += 100;
            println!("item = {}", item)
        }
    }
}

fn main() {
    println!("Hello, world!")
}
