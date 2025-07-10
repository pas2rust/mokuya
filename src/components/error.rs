use std::fmt::Debug;

#[derive(Debug, PartialEq, Default)]
pub struct Error<T: Debug + PartialEq + Default> {
    pub kind: T,
    pub description: String,
    pub code: u8,
}

impl<T: Debug + PartialEq + Default> Error<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn kind(&mut self, new: T) -> &mut Self {
        self.kind = new;
        self
    }
    pub fn description<New: Into<String>>(&mut self, new: New) -> &mut Self {
        self.description = new.into();
        self
    }
    pub fn code<New: Into<u8>>(&mut self, new: New) -> &mut Self {
        self.code = new.into();
        self
    }

    pub fn print(&self) {
        use colorful::Colorful;
        let message = format!(
            "({} {} {}:{}) @ERROR => {:#?}",
            chrono::Local::now(),
            file!(),
            line!(),
            column!(),
            &self,
        )
        .rgb(255, 49, 49);
        println!("{message}")
    }
}
