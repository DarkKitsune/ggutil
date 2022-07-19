pub mod handle_map;
pub mod unique;
pub mod maybe_owned;
pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn unique() {
        // Create two unique handles
        let unique1 = Unique::new();
        let unique2 = Unique::new();
        // Assert that the handles are equal to themselves
        assert_eq!(unique1, unique1);
        assert_eq!(unique2, unique2);
        // Assert that a handle is equal to its clone
        assert_eq!(unique1, unique1.clone());
        // Assert that two individually created handles are not equal
        assert_ne!(unique1, unique2);
    }

    #[test]
    fn handle_map() {
        // Create a handle map
        let mut handle_map = HandleMap::new();
        // Insert a value into the map
        let handle = handle_map.insert(1);
        // Assert that the value is in the map
        assert_eq!(handle_map.get(&handle), Some(&1));
        // Remove the value from the map
        assert_eq!(handle_map.remove(&handle), Some(1));
        // Assert that the value is no longer in the map
        assert_eq!(handle_map.get(&handle), None);
    }

    #[test]
    fn maybe_owned() {
        // Create an owned value
        let mut a = MaybeOwned::Owned(1);
        // Unwrap the value
        assert_eq!(a.clone().unwrap(), Some(1));

        // Borrow the value of `a`
        let b = MaybeOwned::Borrowed(a.as_ref());
        // Unwrap the value
        assert_eq!(b.unwrap(), None);
        
        // Create a new MaybeOwned containing mutable reference to `a`
        let mut c: MaybeOwned<i32> = a.as_mut().unwrap().into();
        // Unwrap a clone of the value
        assert_eq!(c.clone().unwrap(), Some(1));
        // Set the value
        *c.as_mut().unwrap() = 3;
        // Make sure a is now 3
        assert_eq!(a.unwrap(), Some(3));
    }
}
