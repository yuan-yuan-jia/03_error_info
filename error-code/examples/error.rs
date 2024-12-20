use error_code::DeriveToErrorInfo;

#[derive(Debug, thiserror::Error, DeriveToErrorInfo)]
#[error_info(app_type = "http::StatusCode", prefix = "01")]
pub enum MyError {
    #[error("Invalid command: {0}")]
    #[error_info(code = "IC", app_code = "400")]
    InvalidCommand(String),
    #[error("Invalid argument: {0}")]
    #[error_info(code = "IA", app_code = "400", client_msg = "friendly msg")]
    InvalidArgument(String),
    #[error("{0}")]
    #[error_info(code = "RE", app_code = "500")]
    RespError(#[from] std::io::Error),
}

fn main() {
    let err = MyError::InvalidArgument("cmd".to_string());
    let info = err.to_error_info();
    println!("{:?}", info);
}

// use error_code::{ToErrorInfo};
// impl ToErrorInfo for MyError
// {
//     type T = http::StatusCode;
//     fn to_error_info(&self) -> Result<error_code::ErrorInfo<Self::T>, <Self::T as std::str::FromStr>::Err> {
//     match self
//     {
//     MyError::InvalidCommand(_) =>
//     { ErrorInfo::try_new("400", "01IC", "", self ) }, MyError::
//     InvalidArgument(_) =>
//     { ErrorInfo::try_new("400", "01IA", "friendly msg", self ) },
//     MyError::RespError(_) =>
//     { ErrorInfo::try_new("500", "01RE", "", self ) }
//     }
//     }
// }
