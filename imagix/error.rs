pub enum ImagixError {
    FileIOEror(String),
    UserInputError(String),
    ImageResizingError(String),
    FormatError(String),
}
