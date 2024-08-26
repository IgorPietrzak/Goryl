pub mod parse_error;
pub mod runtime_error;
pub mod syntax_error;

pub trait Error {
    fn report(&self);
}
