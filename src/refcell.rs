use std::{cell::UnsafeCell, ops::Deref, ops::DerefMut};

use crate::cell::MyCell;

#[derive(Copy, Clone, PartialEq)]
enum RefState {
    NotBorrowed,
    ImutableBorrow(usize),
    ExclusiveBorrow,
}

struct MyRefCell<T> {
    value: UnsafeCell<T>,
    ref_state: MyCell<RefState>,
}

impl<T> MyRefCell<T>
where
    T: Copy,
{
    fn new(value: T) -> Self {
        MyRefCell {
            value: UnsafeCell::new(value),
            ref_state: MyCell::new(RefState::NotBorrowed),
        }
    }

    fn borrow(&self) -> Option<MyRef<'_, T>> {
        match self.ref_state.get() {
            RefState::ExclusiveBorrow => None,
            RefState::NotBorrowed => {
                self.ref_state.set(RefState::ImutableBorrow(1));
                Some(MyRef::new(self))
            }
            RefState::ImutableBorrow(n) => {
                self.ref_state.set(RefState::ImutableBorrow(n + 1));
                Some(MyRef::new(self))
            }
        }
    }

    fn borrow_mut(&self) -> Option<MyMutRef<'_, T>> {
        if self.ref_state.get() == RefState::NotBorrowed {
            self.ref_state.set(RefState::ExclusiveBorrow);
            Some(MyMutRef::new(self))
        } else {
            None
        }
    }
}

struct MyRef<'t, T> {
    _refcell: &'t MyRefCell<T>,
}

impl<'t, T> MyRef<'t, T> {
    fn new(value: &'t MyRefCell<T>) -> Self {
        MyRef { _refcell: value }
    }
}

impl<T> Deref for MyRef<'_, T>
where
    T: Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self._refcell.value.get() }
    }
}

impl<T> Drop for MyRef<'_, T> {
    fn drop(&mut self) {
        match self._refcell.ref_state.get() {
            RefState::ImutableBorrow(n) => {
                if n == 1 {
                    self._refcell.ref_state.set(RefState::NotBorrowed)
                } else {
                    self._refcell.ref_state.set(RefState::ImutableBorrow(n - 1))
                }
            }
            _ => panic!("Not reachable"),
        }
    }
}

struct MyMutRef<'t, T> {
    _refcell: &'t MyRefCell<T>,
}

impl<'t, T> MyMutRef<'t, T> {
    fn new(value: &'t MyRefCell<T>) -> Self {
        MyMutRef { _refcell: value }
    }
}

impl<T> Deref for MyMutRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self._refcell.value.get() }
    }
}

impl <T> DerefMut for MyMutRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self._refcell.value.get() }
    }
}

impl<T> Drop for MyMutRef<'_, T> {
    fn drop(&mut self) {
        if self._refcell.ref_state.get() == RefState::ExclusiveBorrow {
            self._refcell.ref_state.set(RefState::NotBorrowed)
        } else {
            panic!("Not reachable")
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn give_multiple_immutable_borrow() {
        let my_ref_cell = MyRefCell::new(35);

        let borrow_one = my_ref_cell.borrow();
        let borrow_two = my_ref_cell.borrow();

        assert!(borrow_one.is_some());
        assert!(borrow_two.is_some());
    }

    #[test]
    fn should_not_give_single_mutable_borrow() {
        let my_ref_cell = MyRefCell::new(35);

        let mutable_borrow_one = my_ref_cell.borrow_mut();
        let mutable_borrow_two = my_ref_cell.borrow_mut();

        assert!(mutable_borrow_one.is_some());
        assert!(mutable_borrow_two.is_none());
    }

    #[test]
    fn should_not_give_mutable_borrow_when_there_is_any_immutable_borrow() {
        let my_ref_cell = MyRefCell::new(35);

        let immutable_borrow_one = my_ref_cell.borrow();
        let mutable_borrow = my_ref_cell.borrow_mut();
        let immutable_borrow_two = my_ref_cell.borrow();

        assert!(immutable_borrow_one.is_some());
        assert!(immutable_borrow_two.is_some());
        assert!(mutable_borrow.is_none());
    }
    #[test]
    fn should_not_give_immutable_borrow_when_there_is_any_mutable_borrow() {
        let my_ref_cell = MyRefCell::new(35);

        let mutable_borrow = my_ref_cell.borrow_mut();
        let immutable_borrow = my_ref_cell.borrow();

        assert!(immutable_borrow.is_none());
        assert!(mutable_borrow.is_some());
    }
}
