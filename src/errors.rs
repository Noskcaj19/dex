use std::error::Error;

use serenity;

#[derive(Fail, Debug)]
#[fail(display = "Unable to locate user home directory")]
pub struct HomeDirError;

#[derive(Fail, Debug)]
#[fail(display = "Must be run in a interactive terminal")]
pub struct NonInteractiveTty;

#[derive(Fail, Debug)]
#[fail(display = "Error originating from Serentity:\n{}", _0)]
pub struct InternalSerenityError(pub String);

impl From<serenity::Error> for InternalSerenityError {
    fn from(err: serenity::Error) -> InternalSerenityError {
        InternalSerenityError(err.description().into())
    }
}
