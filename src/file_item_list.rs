pub mod directory_item;
pub mod file_item;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Kinds {
    File = 1,
    Directory = 2,
    Hidden = 3,
}
