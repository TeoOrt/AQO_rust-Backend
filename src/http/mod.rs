pub use method::Method;
pub use request::ParseError;
pub use request::Request;
pub mod method;
pub mod query_string;
pub mod request;
pub use query_string::{QueryString, Value as QueryStringValue};
//add all the packages into a package
pub mod response;
pub use response::Response;
pub mod status_code;
pub use status_code::StatusCode;
