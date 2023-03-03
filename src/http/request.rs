use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
//to get data from Method
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

//example resquest is
// GET /search?name=abc&sort=1 HTTP/1.1
impl TryFrom<&[u8]> for Request {
    type Error = ParseError; //so it will return an error string enum instead of just a  string
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        /*
        this section transform the bite slice into utf8 if error
        then return Error ParserError other wise return the unwrapped portion
        of the  code  so return Result<Self>
        */

        //converted byte slice into string slice string slice is &str[];
        let request = str::from_utf8(value)?;

        unimplemented!()
    }
}

//helper function
//@params Input is the string of the request,
//@output is a tuple with the characteristic of the string and the actual string
fn get_next_word(request: &str) -> (&str, &str) {
    unimplemented!()
}

//enums are like int, char, but for especific things
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

//this is creating an implematation for the message
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

//Needed when error checking for conversion of Utf8Errror
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
//this is needed for the TryFRom implementation
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
//Same thing for this implementation, in order for us to use the implementation\
// we need to extended this parts of the class and give it our own specific needs
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
//DIsplay and Debug are needed to implement Error into ParseError
impl Error for ParseError {}
