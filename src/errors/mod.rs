pub mod syntax_error;
pub trait Error {
    fn report(&self);
}
