mod identifier;
mod number;
mod whitespace;
mod comment;

pub(crate) use identifier::read_identifier;
pub(crate) use number::read_number;
pub(crate) use whitespace::skip_whitespace;
pub(crate) use comment::skip_comment;
