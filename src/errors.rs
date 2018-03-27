#[derive(Fail, Debug)]
#[fail(display = "Unable to locate user home directory")]
pub struct HomeDirError;

#[derive(Fail, Debug)]
#[fail(display = "Error originating from Serentity:\n{}", _0)]
pub struct InternalSerenityError(pub String);

impl From<::serenity::Error> for InternalSerenityError {
    fn from(err: ::serenity::Error) -> InternalSerenityError {
        use std::error::Error;
        InternalSerenityError(err.description().into())
    }
}
