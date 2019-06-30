#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralMessage {
    #[prost(int64, tag="1")]
    pub time: i64,
    #[prost(oneof="general_message::Payload", tags="2, 3")]
    pub payload: ::std::option::Option<general_message::Payload>,
}
pub mod general_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="2")]
        AuthRequest(super::super::auth::AuthRequest),
        #[prost(message, tag="3")]
        AuthResponse(super::super::auth::AuthResponse),
    }
}
