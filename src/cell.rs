use core::cell::UnsafeCell;

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> where T : Copy {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    {
        unsafe { *self.value.get().clone() }
    }

    pub fn set(&self, value: T) {
        {
            unsafe { *self.value.get() = value }
        }
    }

    pub fn replace(&self, value: T) -> T {
        unsafe {
            let original_value = *self.value.get().clone();
            *self.value.get() = value;
            original_value
        }
    }

    fn swap(&self, other: &MyCell<T>) {
        self.set(other.replace(self.get()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_value_of_cell() {
        let cell = MyCell::new(32);
        assert_eq!(cell.get(), 32);
    }

    #[test]
    fn set_value_in_cell() {
        let cell = MyCell::new(32);

        assert_eq!(cell.get(), 32);
        cell.set(35);
        assert_eq!(cell.get(), 35);
        cell.set(45);
        assert_eq!(cell.get(), 45);
    }

    #[test]
    fn replace_value_in_cell() {
        let cell = MyCell::new(32);

        assert_eq!(cell.replace(30), 32);
        assert_eq!(cell.get(), 30);
    }

    #[test]
    fn swap_value_of_cells() {
        let cell_one = MyCell::new(40);
        let cell_two = MyCell::new(50);

        cell_one.swap(&cell_two);

        assert_eq!(cell_one.get(), 50);
        assert_eq!(cell_two.get(), 40);
    }
}
