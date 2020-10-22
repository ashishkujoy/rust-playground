mod cell;
mod list;
mod list2;
mod rc;
mod refcell;
pub mod request;
pub mod response;
pub mod server;

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    fn it_works() {
        let cell = Cell::new(43);
        cell.swap(&Cell::new(34));
        assert_eq!(2 + 2, 4);
    }
}
