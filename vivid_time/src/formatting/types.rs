use std::fmt;

pub trait Formatter<T: ?Sized> {
    fn fmt<W: fmt::Write>(&self, buffer: &mut W, data: &T) -> fmt::Result;

    fn format(&self, data: &T) -> Result<String, fmt::Error> {
        let mut buffer = String::new();
        self.fmt(&mut buffer, data)?;
        Ok(buffer)
    }
}
