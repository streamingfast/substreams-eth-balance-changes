// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChanges {
    #[prost(message, repeated, tag="1")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub old_value: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_value: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub reason: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
    /// if the change is done in a transaction
    #[prost(string, tag="6")]
    pub transaction: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
