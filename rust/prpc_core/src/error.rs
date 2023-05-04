use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PRPCError {
    pub kind: PRPCErrorType,
    pub message: String,
}


impl serde::ser::Serialize for PRPCErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u8(self.to_code())
    }
}

struct PRPCErrorTypeVisitor;

impl<'de> serde::de::Visitor<'de> for PRPCErrorTypeVisitor {
    type Value = PRPCErrorType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 1 and 16 inclusive")
    }

    // TODO: Figure out how to remove the duplication here

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            // Leaving this as unknown, an alternative would be to return an error
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            1 => Ok(PRPCErrorType::Cancelled),
            3 => Ok(PRPCErrorType::InvalidArgument),
            4 => Ok(PRPCErrorType::DeadlineExceeded),
            5 => Ok(PRPCErrorType::NotFound),
            6 => Ok(PRPCErrorType::AlreadyExists),
            7 => Ok(PRPCErrorType::PermissionDenied),
            8 => Ok(PRPCErrorType::ResourceExhausted),
            9 => Ok(PRPCErrorType::FailedPrecondition),
            10 => Ok(PRPCErrorType::Aborted),
            11 => Ok(PRPCErrorType::OutOfRange),
            12 => Ok(PRPCErrorType::Unimplemented),
            13 => Ok(PRPCErrorType::Internal),
            14 => Ok(PRPCErrorType::Unavailable),
            15 => Ok(PRPCErrorType::DataLoss),
            16 => Ok(PRPCErrorType::Unauthenticated),
            _ => Ok(PRPCErrorType::Unknown),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for PRPCErrorType {
    fn deserialize<D>(deserializer: D) -> Result<PRPCErrorType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(PRPCErrorTypeVisitor)
    }
}

pub enum PRPCErrorType {
    Cancelled,
    Unknown,
    InvalidArgument,
    DeadlineExceeded,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    ResourceExhausted,
    FailedPrecondition,
    Aborted,
    OutOfRange,
    Unimplemented,
    Internal,
    Unavailable,
    DataLoss,
    Unauthenticated,
}
impl PRPCErrorType {
    pub fn to_code(&self) -> u8 {
        match self {
            Self::Cancelled => 1,
            Self::Unknown => 2,
            Self::InvalidArgument => 3,
            Self::DeadlineExceeded => 4,
            Self::NotFound => 5,
            Self::AlreadyExists => 6,
            Self::PermissionDenied => 7,
            Self::ResourceExhausted => 8,
            Self::FailedPrecondition => 9,
            Self::Aborted => 10,
            Self::OutOfRange => 11,
            Self::Unimplemented => 12,
            Self::Internal => 13,
            Self::Unavailable => 14,
            Self::DataLoss => 15,
            Self::Unauthenticated => 16,
        }
    }
}

impl Into<u8> for PRPCErrorType {
    fn into(self) -> u8 {
        self.to_code()
    }
}

impl Into<u8> for &PRPCErrorType {
    fn into(self) -> u8 {
        self.to_code()
    }
}

impl Into<u8> for &mut PRPCErrorType {
    fn into(self) -> u8 {
        self.to_code()
    }
}
