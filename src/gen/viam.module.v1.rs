// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddResourceRequest {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<super::super::app::v1::ComponentConfig>,
    #[prost(string, repeated, tag="2")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddResourceResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconfigureResourceRequest {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<super::super::app::v1::ComponentConfig>,
    #[prost(string, repeated, tag="2")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconfigureResourceResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveResourceRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveResourceResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandlerDefinition {
    #[prost(message, optional, tag="1")]
    pub subtype: ::core::option::Option<super::super::robot::v1::ResourceRpcSubtype>,
    #[prost(string, repeated, tag="2")]
    pub models: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandlerMap {
    #[prost(message, repeated, tag="1")]
    pub handlers: ::prost::alloc::vec::Vec<HandlerDefinition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyRequest {
    #[prost(string, tag="1")]
    pub parent_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    #[prost(bool, tag="1")]
    pub ready: bool,
    #[prost(message, optional, tag="2")]
    pub handlermap: ::core::option::Option<HandlerMap>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateConfigRequest {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<super::super::app::v1::ComponentConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateConfigResponse {
    #[prost(string, repeated, tag="1")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `viam.module.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x99, 0x1a, 0x0a, 0x16, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x6d,
    0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x76, 0x69, 0x61,
    0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x12, 0x61, 0x70, 0x70,
    0x2f, 0x76, 0x31, 0x2f, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x14, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2f, 0x76, 0x31, 0x2f, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6e, 0x0a, 0x12, 0x41, 0x64, 0x64, 0x52, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x34, 0x0a, 0x06, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x6e,
    0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x12, 0x22, 0x0a, 0x0c, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x63, 0x69, 0x65,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65,
    0x6e, 0x63, 0x69, 0x65, 0x73, 0x22, 0x15, 0x0a, 0x13, 0x41, 0x64, 0x64, 0x52, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x76, 0x0a, 0x1a,
    0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x34, 0x0a, 0x06, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x76, 0x69, 0x61,
    0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x12, 0x22, 0x0a, 0x0c, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e,
    0x63, 0x69, 0x65, 0x73, 0x22, 0x1d, 0x0a, 0x1b, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x75, 0x72, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0x2b, 0x0a, 0x15, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x22, 0x18, 0x0a, 0x16, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x68, 0x0a, 0x11, 0x48, 0x61,
    0x6e, 0x64, 0x6c, 0x65, 0x72, 0x44, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x3b, 0x0a, 0x07, 0x73, 0x75, 0x62, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x21, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e, 0x76, 0x31,
    0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x50, 0x43, 0x53, 0x75, 0x62, 0x74,
    0x79, 0x70, 0x65, 0x52, 0x07, 0x73, 0x75, 0x62, 0x74, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x6f,
    0x64, 0x65, 0x6c, 0x73, 0x22, 0x4b, 0x0a, 0x0a, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x4d,
    0x61, 0x70, 0x12, 0x3d, 0x0a, 0x08, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x44, 0x65, 0x66,
    0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x08, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72,
    0x73, 0x22, 0x35, 0x0a, 0x0c, 0x52, 0x65, 0x61, 0x64, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x25, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x70, 0x61, 0x72, 0x65, 0x6e,
    0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x22, 0x61, 0x0a, 0x0d, 0x52, 0x65, 0x61, 0x64,
    0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x65, 0x61,
    0x64, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x72, 0x65, 0x61, 0x64, 0x79, 0x12,
    0x3a, 0x0a, 0x0a, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x6d, 0x61, 0x70, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c,
    0x65, 0x2e, 0x76, 0x31, 0x2e, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x4d, 0x61, 0x70, 0x52,
    0x0a, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x6d, 0x61, 0x70, 0x22, 0x4d, 0x0a, 0x15, 0x56,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x34, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0x3c, 0x0a, 0x16, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x22, 0x0a, 0x0c, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e,
    0x63, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x64, 0x65, 0x70, 0x65,
    0x6e, 0x64, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x32, 0xdf, 0x03, 0x0a, 0x0d, 0x4d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x56, 0x0a, 0x0b, 0x41, 0x64,
    0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x22, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x64, 0x64, 0x52, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e,
    0x76, 0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x41,
    0x64, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x6e, 0x0a, 0x13, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x2a, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2b, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x5f, 0x0a, 0x0e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x12, 0x25, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x26, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x6d,
    0x6f, 0x76, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x44, 0x0a, 0x05, 0x52, 0x65, 0x61, 0x64, 0x79, 0x12, 0x1c, 0x2e, 0x76,
    0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65,
    0x61, 0x64, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1d, 0x2e, 0x76, 0x69, 0x61,
    0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x61, 0x64,
    0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5f, 0x0a, 0x0e, 0x56, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x25, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x26, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65,
    0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x1b, 0x5a, 0x19, 0x67, 0x6f,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x6d, 0x6f,
    0x64, 0x75, 0x6c, 0x65, 0x2f, 0x76, 0x31, 0x4a, 0xfa, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x43, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x1e, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x07, 0x00, 0x30, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x07, 0x00,
    0x30, 0x0a, 0x44, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x1a, 0x01, 0x1a, 0x38, 0x20,
    0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65,
    0x61, 0x6c, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x61, 0x72,
    0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x15, 0x0a, 0x48, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x44,
    0x1a, 0x3b, 0x20, 0x41, 0x64, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x74,
    0x65, 0x6c, 0x6c, 0x73, 0x20, 0x61, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x61, 0x62,
    0x6f, 0x75, 0x74, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0c, 0x2f, 0x42, 0x0a, 0x55, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0f, 0x02, 0x5c, 0x1a, 0x48, 0x20, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x73,
    0x20, 0x61, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x69, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x1a, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x3f, 0x5a, 0x0a, 0x5c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x12, 0x02, 0x4d, 0x1a, 0x4f, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x52, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x61, 0x20, 0x6d,
    0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x2f, 0x73,
    0x74, 0x6f, 0x70, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x6d, 0x6f,
    0x76, 0x65, 0x20, 0x69, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x12, 0x06, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12,
    0x15, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x35, 0x4b,
    0x0a, 0x66, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x03, 0x15, 0x02, 0x32, 0x1a, 0x59, 0x20,
    0x52, 0x65, 0x61, 0x64, 0x79, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x73,
    0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x69,
    0x73, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65,
    0x61, 0x64, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x63, 0x69, 0x65, 0x76, 0x65, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x15, 0x06, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x15, 0x0c, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x15,
    0x23, 0x30, 0x0a, 0x78, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19, 0x02, 0x4d, 0x1a,
    0x6b, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x20, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x73, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x0a, 0x20, 0x64, 0x65,
    0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x06, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x19, 0x15, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x19, 0x35, 0x4b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1c, 0x00,
    0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1d, 0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1d, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x21, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x20, 0x00, 0x1e, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08,
    0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x22, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x23, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x23, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23,
    0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x24, 0x21, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x00, 0x26,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x26, 0x08, 0x23, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x10, 0x11, 0x0a, 0x09, 0x0a, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x2b, 0x00, 0x21, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2b,
    0x08, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2d, 0x00, 0x30, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x2e, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2e, 0x23, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x2d,
    0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x2f, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x31, 0x00,
    0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x31, 0x08, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x32, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x32, 0x1d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x32, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x35, 0x00, 0x37, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x35, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x36, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x36, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36,
    0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x38, 0x00, 0x3b, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x38, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x00, 0x12, 0x03, 0x39, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x39, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x39, 0x07, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x0f,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x02, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3a, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04,
    0x3d, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x02, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3e, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x3e, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x41,
    0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x41, 0x08, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x42, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x42, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x42, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x42, 0x21, 0x22, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.module.v1.tonic.rs");
// @@protoc_insertion_point(module)