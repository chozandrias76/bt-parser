use nom::{bytes::complete::tag, IResult};

use crate::types::nested::Nested;

#[derive(Debug)]
enum TypedefMember {
    Char,
    Byte,
    UChar,
    UByte,
    Short,
    Int16,
    Ushort,
    UInt16,
    Word,
    Int,
    Int32,
    Long,
    Uint,
    UnInt32,
    ULong,
    Dword,
    Int64,
    Quad,
    UInt64,
    UQuad,
    Float,
    Double,
    HFloat,
    DOSDate,
    DOSTime,
    FileTime,
    OLETime,
    TimeT,
    Time64T,
    String,
    WCharT,
    WString,
    GUID,
    Opcode,
    Struct,
    Unsigned,
    Enum,
}

impl TypedefMember {
    pub fn iterator() -> std::slice::Iter<'static, TypedefMember> {
        static MEMBERS: [TypedefMember; 37] = [
            TypedefMember::Char,
            TypedefMember::Byte,
            TypedefMember::UChar,
            TypedefMember::UByte,
            TypedefMember::Short,
            TypedefMember::Int16,
            TypedefMember::Ushort,
            TypedefMember::UInt16,
            TypedefMember::Word,
            TypedefMember::Int,
            TypedefMember::Int32,
            TypedefMember::Long,
            TypedefMember::Uint,
            TypedefMember::UnInt32,
            TypedefMember::ULong,
            TypedefMember::Dword,
            TypedefMember::Int64,
            TypedefMember::Quad,
            TypedefMember::UInt64,
            TypedefMember::UQuad,
            TypedefMember::Float,
            TypedefMember::Double,
            TypedefMember::HFloat,
            TypedefMember::DOSDate,
            TypedefMember::DOSTime,
            TypedefMember::FileTime,
            TypedefMember::OLETime,
            TypedefMember::TimeT,
            TypedefMember::Time64T,
            TypedefMember::String,
            TypedefMember::WCharT,
            TypedefMember::WString,
            TypedefMember::GUID,
            TypedefMember::Opcode,
            TypedefMember::Struct,
            TypedefMember::Unsigned,
            TypedefMember::Enum,
        ];
        MEMBERS.iter()
    }
}

impl std::fmt::Display for TypedefMember {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypedefMember::Char => write!(f, "char"),
            TypedefMember::Byte => write!(f, "byte"),
            TypedefMember::UChar => write!(f, "uchar"),
            TypedefMember::UByte => write!(f, "ubyte"),
            TypedefMember::Short => write!(f, "short"),
            TypedefMember::Int16 => write!(f, "int16"),
            TypedefMember::Ushort => write!(f, "ushort"),
            TypedefMember::UInt16 => write!(f, "uint16"),
            TypedefMember::Word => write!(f, "word"),
            TypedefMember::Int => write!(f, "int"),
            TypedefMember::Int32 => write!(f, "int32"),
            TypedefMember::Long => write!(f, "long"),
            TypedefMember::Uint => write!(f, "uint"),
            TypedefMember::UnInt32 => write!(f, "uint32"),
            TypedefMember::ULong => write!(f, "ulong"),
            TypedefMember::Dword => write!(f, "dword"),
            TypedefMember::Int64 => write!(f, "int64"),
            TypedefMember::Quad => write!(f, "quad"),
            TypedefMember::UInt64 => write!(f, "uint64"),
            TypedefMember::UQuad => write!(f, "uquad"),
            TypedefMember::Float => write!(f, "float"),
            TypedefMember::Double => write!(f, "double"),
            TypedefMember::HFloat => write!(f, "hfloat"),
            TypedefMember::DOSDate => write!(f, "dosdate"),
            TypedefMember::DOSTime => write!(f, "dostime"),
            TypedefMember::FileTime => write!(f, "filetime"),
            TypedefMember::OLETime => write!(f, "oletime"),
            TypedefMember::TimeT => write!(f, "timet"),
            TypedefMember::Time64T => write!(f, "time64t"),
            TypedefMember::String => write!(f, "string"),
            TypedefMember::WCharT => write!(f, "wchar_t"),
            TypedefMember::WString => write!(f, "wstring"),
            TypedefMember::GUID => write!(f, "guid"),
            TypedefMember::Opcode => write!(f, "opcode"),
            TypedefMember::Struct => write!(f, "struct"),
            TypedefMember::Unsigned => write!(f, "unsigned"),
            TypedefMember::Enum => write!(f, "enum"),
        }
    }
}

fn parse_typedef_member(input: &str) -> IResult<&str, &str> {
    for member in TypedefMember::iterator() {
        let result = tag(member.to_string().as_str())(input);
        if result.is_ok() {
            return result;
        }
    }
    // Return an error if none of the members match
    return Err(nom::Err::Error(nom::error_position!(
        input,
        nom::error::ErrorKind::Fail
    )));
}

pub fn typedef_member(input: &str) -> IResult<&str, Nested> {
    match parse_typedef_member(input) {
        Ok((rest, member)) => Ok((rest.trim_start(), Nested::Text(member.to_string()))),
        Err(_) => Err(nom::Err::Error(nom::error_position!(
            input,
            nom::error::ErrorKind::Fail
        ))),
    }
}

#[cfg(test)]
mod typedef_member_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_typedef_member1() {
        // Test that all members can be parsed
        for member in TypedefMember::iterator() {
            let member_string = member.to_string();
            let input = member_string.as_str();
            let result = typedef_member(input);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_typedef_member2() {
        // Test that an invalid member cannot be parsed
        let input = "invalid_member";
        let result = typedef_member(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_typedef_member3() {
        // Test that a valid member can be parsed
        let input = "unsigned int myInt;";
        let result = typedef_member(input);
        assert!(result.is_ok());
        let (rest, member) = result.unwrap();
        assert_eq!(rest, "int myInt;");
        assert_eq!(member, "unsigned".into());
    }
}