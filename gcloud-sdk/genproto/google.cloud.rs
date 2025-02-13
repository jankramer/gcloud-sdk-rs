/// An enum to be used to mark the essential (for polling) fields in an
/// API-specific Operation object. A custom Operation object may contain many
/// different fields, but only few of them are essential to conduct a successful
/// polling process.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationResponseMapping {
    /// Do not use.
    Undefined = 0,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.name.
    Name = 1,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.done. If the annotated field is of
    /// an enum type, `annotated_field_name == EnumType.DONE` semantics should be
    /// equivalent to `Operation.done == true`. If the annotated field is of type
    /// boolean, then it should follow the same semantics as Operation.done.
    /// Otherwise, a non-empty value should be treated as `Operation.done == true`.
    Status = 2,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.error.code.
    ErrorCode = 3,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.error.message.
    ErrorMessage = 4,
}
impl OperationResponseMapping {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationResponseMapping::Undefined => "UNDEFINED",
            OperationResponseMapping::Name => "NAME",
            OperationResponseMapping::Status => "STATUS",
            OperationResponseMapping::ErrorCode => "ERROR_CODE",
            OperationResponseMapping::ErrorMessage => "ERROR_MESSAGE",
        }
    }
}
