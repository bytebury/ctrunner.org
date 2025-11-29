pub mod htmx;
pub mod pagination;
pub mod rbac;
pub mod validation;

// String Utilities

pub trait StringExt {
    fn is_whitespace_or_empty(&self) -> bool;
}

impl StringExt for String {
    fn is_whitespace_or_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

impl StringExt for str {
    fn is_whitespace_or_empty(&self) -> bool {
        self.trim().is_empty()
    }
}
