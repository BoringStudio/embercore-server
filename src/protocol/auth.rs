#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(string, tag="1")]
    pub login: std::string::String,
    #[prost(string, tag="2")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResponse {
    #[prost(enumeration="auth_response::Status", tag="1")]
    pub status: i32,
    #[prost(message, optional, tag="2")]
    pub result: ::std::option::Option<auth_response::Result>,
}
pub mod auth_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(uint32, tag="1")]
        pub user_id: u32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Success = 0,
        InvalidCredentials = 1,
        ServerIsFull = 2,
        Banned = 3,
    }
}
