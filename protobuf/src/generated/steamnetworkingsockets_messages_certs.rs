// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `steamnetworkingsockets_messages_certs.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:CMsgSteamNetworkingIdentityLegacyBinary)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMsgSteamNetworkingIdentityLegacyBinary {
    // message fields
    // @@protoc_insertion_point(field:CMsgSteamNetworkingIdentityLegacyBinary.steam_id)
    pub steam_id: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CMsgSteamNetworkingIdentityLegacyBinary.xbox_pairwise_id)
    pub xbox_pairwise_id: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CMsgSteamNetworkingIdentityLegacyBinary.generic_bytes)
    pub generic_bytes: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:CMsgSteamNetworkingIdentityLegacyBinary.generic_string)
    pub generic_string: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CMsgSteamNetworkingIdentityLegacyBinary.ipv6_and_port)
    pub ipv6_and_port: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:CMsgSteamNetworkingIdentityLegacyBinary.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMsgSteamNetworkingIdentityLegacyBinary {
    fn default() -> &'a CMsgSteamNetworkingIdentityLegacyBinary {
        <CMsgSteamNetworkingIdentityLegacyBinary as ::protobuf::Message>::default_instance()
    }
}

impl CMsgSteamNetworkingIdentityLegacyBinary {
    pub fn new() -> CMsgSteamNetworkingIdentityLegacyBinary {
        ::std::default::Default::default()
    }

    // optional fixed64 steam_id = 16;

    pub fn steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    // optional string xbox_pairwise_id = 17;

    pub fn xbox_pairwise_id(&self) -> &str {
        match self.xbox_pairwise_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_xbox_pairwise_id(&mut self) {
        self.xbox_pairwise_id = ::std::option::Option::None;
    }

    pub fn has_xbox_pairwise_id(&self) -> bool {
        self.xbox_pairwise_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xbox_pairwise_id(&mut self, v: ::std::string::String) {
        self.xbox_pairwise_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xbox_pairwise_id(&mut self) -> &mut ::std::string::String {
        if self.xbox_pairwise_id.is_none() {
            self.xbox_pairwise_id = ::std::option::Option::Some(::std::string::String::new());
        }
        self.xbox_pairwise_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_xbox_pairwise_id(&mut self) -> ::std::string::String {
        self.xbox_pairwise_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bytes generic_bytes = 2;

    pub fn generic_bytes(&self) -> &[u8] {
        match self.generic_bytes.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_generic_bytes(&mut self) {
        self.generic_bytes = ::std::option::Option::None;
    }

    pub fn has_generic_bytes(&self) -> bool {
        self.generic_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generic_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.generic_bytes = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_generic_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.generic_bytes.is_none() {
            self.generic_bytes = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.generic_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_generic_bytes(&mut self) -> ::std::vec::Vec<u8> {
        self.generic_bytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional string generic_string = 3;

    pub fn generic_string(&self) -> &str {
        match self.generic_string.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_generic_string(&mut self) {
        self.generic_string = ::std::option::Option::None;
    }

    pub fn has_generic_string(&self) -> bool {
        self.generic_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generic_string(&mut self, v: ::std::string::String) {
        self.generic_string = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_generic_string(&mut self) -> &mut ::std::string::String {
        if self.generic_string.is_none() {
            self.generic_string = ::std::option::Option::Some(::std::string::String::new());
        }
        self.generic_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_generic_string(&mut self) -> ::std::string::String {
        self.generic_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bytes ipv6_and_port = 4;

    pub fn ipv6_and_port(&self) -> &[u8] {
        match self.ipv6_and_port.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_ipv6_and_port(&mut self) {
        self.ipv6_and_port = ::std::option::Option::None;
    }

    pub fn has_ipv6_and_port(&self) -> bool {
        self.ipv6_and_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipv6_and_port(&mut self, v: ::std::vec::Vec<u8>) {
        self.ipv6_and_port = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ipv6_and_port(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ipv6_and_port.is_none() {
            self.ipv6_and_port = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.ipv6_and_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_ipv6_and_port(&mut self) -> ::std::vec::Vec<u8> {
        self.ipv6_and_port.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CMsgSteamNetworkingIdentityLegacyBinary {
    const NAME: &'static str = "CMsgSteamNetworkingIdentityLegacyBinary";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                129 => {
                    self.steam_id = ::std::option::Option::Some(is.read_fixed64()?);
                },
                138 => {
                    self.xbox_pairwise_id = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    self.generic_bytes = ::std::option::Option::Some(is.read_bytes()?);
                },
                26 => {
                    self.generic_string = ::std::option::Option::Some(is.read_string()?);
                },
                34 => {
                    self.ipv6_and_port = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 2 + 8;
        }
        if let Some(v) = self.xbox_pairwise_id.as_ref() {
            my_size += ::protobuf::rt::string_size(17, &v);
        }
        if let Some(v) = self.generic_bytes.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.generic_string.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.ipv6_and_port.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(16, v)?;
        }
        if let Some(v) = self.xbox_pairwise_id.as_ref() {
            os.write_string(17, v)?;
        }
        if let Some(v) = self.generic_bytes.as_ref() {
            os.write_bytes(2, v)?;
        }
        if let Some(v) = self.generic_string.as_ref() {
            os.write_string(3, v)?;
        }
        if let Some(v) = self.ipv6_and_port.as_ref() {
            os.write_bytes(4, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CMsgSteamNetworkingIdentityLegacyBinary {
        CMsgSteamNetworkingIdentityLegacyBinary::new()
    }

    fn clear(&mut self) {
        self.steam_id = ::std::option::Option::None;
        self.xbox_pairwise_id = ::std::option::Option::None;
        self.generic_bytes = ::std::option::Option::None;
        self.generic_string = ::std::option::Option::None;
        self.ipv6_and_port = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMsgSteamNetworkingIdentityLegacyBinary {
        static instance: CMsgSteamNetworkingIdentityLegacyBinary = CMsgSteamNetworkingIdentityLegacyBinary {
            steam_id: ::std::option::Option::None,
            xbox_pairwise_id: ::std::option::Option::None,
            generic_bytes: ::std::option::Option::None,
            generic_string: ::std::option::Option::None,
            ipv6_and_port: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CMsgSteamDatagramCertificate)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMsgSteamDatagramCertificate {
    // message fields
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.key_type)
    pub key_type: ::std::option::Option<::protobuf::EnumOrUnknown<cmsg_steam_datagram_certificate::EKeyType>>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.key_data)
    pub key_data: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.legacy_steam_id)
    pub legacy_steam_id: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.legacy_identity_binary)
    pub legacy_identity_binary: ::protobuf::MessageField<CMsgSteamNetworkingIdentityLegacyBinary>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.identity_string)
    pub identity_string: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.gameserver_datacenter_ids)
    pub gameserver_datacenter_ids: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.time_created)
    pub time_created: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.time_expiry)
    pub time_expiry: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.app_ids)
    pub app_ids: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificate.ip_addresses)
    pub ip_addresses: ::std::vec::Vec<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:CMsgSteamDatagramCertificate.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMsgSteamDatagramCertificate {
    fn default() -> &'a CMsgSteamDatagramCertificate {
        <CMsgSteamDatagramCertificate as ::protobuf::Message>::default_instance()
    }
}

impl CMsgSteamDatagramCertificate {
    pub fn new() -> CMsgSteamDatagramCertificate {
        ::std::default::Default::default()
    }

    // optional .CMsgSteamDatagramCertificate.EKeyType key_type = 1;

    pub fn key_type(&self) -> cmsg_steam_datagram_certificate::EKeyType {
        match self.key_type {
            Some(e) => e.enum_value_or(cmsg_steam_datagram_certificate::EKeyType::INVALID),
            None => cmsg_steam_datagram_certificate::EKeyType::INVALID,
        }
    }

    pub fn clear_key_type(&mut self) {
        self.key_type = ::std::option::Option::None;
    }

    pub fn has_key_type(&self) -> bool {
        self.key_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_type(&mut self, v: cmsg_steam_datagram_certificate::EKeyType) {
        self.key_type = ::std::option::Option::Some(::protobuf::EnumOrUnknown::new(v));
    }

    // optional bytes key_data = 2;

    pub fn key_data(&self) -> &[u8] {
        match self.key_data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_key_data(&mut self) {
        self.key_data = ::std::option::Option::None;
    }

    pub fn has_key_data(&self) -> bool {
        self.key_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_data = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key_data.is_none() {
            self.key_data = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.key_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_data(&mut self) -> ::std::vec::Vec<u8> {
        self.key_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional fixed64 legacy_steam_id = 4;

    pub fn legacy_steam_id(&self) -> u64 {
        self.legacy_steam_id.unwrap_or(0)
    }

    pub fn clear_legacy_steam_id(&mut self) {
        self.legacy_steam_id = ::std::option::Option::None;
    }

    pub fn has_legacy_steam_id(&self) -> bool {
        self.legacy_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_steam_id(&mut self, v: u64) {
        self.legacy_steam_id = ::std::option::Option::Some(v);
    }

    // optional string identity_string = 12;

    pub fn identity_string(&self) -> &str {
        match self.identity_string.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_identity_string(&mut self) {
        self.identity_string = ::std::option::Option::None;
    }

    pub fn has_identity_string(&self) -> bool {
        self.identity_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identity_string(&mut self, v: ::std::string::String) {
        self.identity_string = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identity_string(&mut self) -> &mut ::std::string::String {
        if self.identity_string.is_none() {
            self.identity_string = ::std::option::Option::Some(::std::string::String::new());
        }
        self.identity_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_identity_string(&mut self) -> ::std::string::String {
        self.identity_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional fixed32 time_created = 8;

    pub fn time_created(&self) -> u32 {
        self.time_created.unwrap_or(0)
    }

    pub fn clear_time_created(&mut self) {
        self.time_created = ::std::option::Option::None;
    }

    pub fn has_time_created(&self) -> bool {
        self.time_created.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_created(&mut self, v: u32) {
        self.time_created = ::std::option::Option::Some(v);
    }

    // optional fixed32 time_expiry = 9;

    pub fn time_expiry(&self) -> u32 {
        self.time_expiry.unwrap_or(0)
    }

    pub fn clear_time_expiry(&mut self) {
        self.time_expiry = ::std::option::Option::None;
    }

    pub fn has_time_expiry(&self) -> bool {
        self.time_expiry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_expiry(&mut self, v: u32) {
        self.time_expiry = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for CMsgSteamDatagramCertificate {
    const NAME: &'static str = "CMsgSteamDatagramCertificate";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.key_type = ::std::option::Option::Some(is.read_enum_or_unknown()?);
                },
                18 => {
                    self.key_data = ::std::option::Option::Some(is.read_bytes()?);
                },
                33 => {
                    self.legacy_steam_id = ::std::option::Option::Some(is.read_fixed64()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.legacy_identity_binary)?;
                },
                98 => {
                    self.identity_string = ::std::option::Option::Some(is.read_string()?);
                },
                42 => {
                    is.read_repeated_packed_fixed32_into(&mut self.gameserver_datacenter_ids)?;
                },
                45 => {
                    self.gameserver_datacenter_ids.push(is.read_fixed32()?);
                },
                69 => {
                    self.time_created = ::std::option::Option::Some(is.read_fixed32()?);
                },
                77 => {
                    self.time_expiry = ::std::option::Option::Some(is.read_fixed32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.app_ids)?;
                },
                80 => {
                    self.app_ids.push(is.read_uint32()?);
                },
                106 => {
                    self.ip_addresses.push(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.key_type {
            my_size += ::protobuf::rt::int32_size(1, v.value());
        }
        if let Some(v) = self.key_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.legacy_steam_id {
            my_size += 1 + 8;
        }
        if let Some(v) = self.legacy_identity_binary.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.identity_string.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        my_size += 5 * self.gameserver_datacenter_ids.len() as u64;
        if let Some(v) = self.time_created {
            my_size += 1 + 4;
        }
        if let Some(v) = self.time_expiry {
            my_size += 1 + 4;
        }
        for value in &self.app_ids {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        for value in &self.ip_addresses {
            my_size += ::protobuf::rt::string_size(13, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.key_type {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&v))?;
        }
        if let Some(v) = self.key_data.as_ref() {
            os.write_bytes(2, v)?;
        }
        if let Some(v) = self.legacy_steam_id {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.legacy_identity_binary.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.identity_string.as_ref() {
            os.write_string(12, v)?;
        }
        for v in &self.gameserver_datacenter_ids {
            os.write_fixed32(5, *v)?;
        };
        if let Some(v) = self.time_created {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.time_expiry {
            os.write_fixed32(9, v)?;
        }
        for v in &self.app_ids {
            os.write_uint32(10, *v)?;
        };
        for v in &self.ip_addresses {
            os.write_string(13, &v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CMsgSteamDatagramCertificate {
        CMsgSteamDatagramCertificate::new()
    }

    fn clear(&mut self) {
        self.key_type = ::std::option::Option::None;
        self.key_data = ::std::option::Option::None;
        self.legacy_steam_id = ::std::option::Option::None;
        self.legacy_identity_binary.clear();
        self.identity_string = ::std::option::Option::None;
        self.gameserver_datacenter_ids.clear();
        self.time_created = ::std::option::Option::None;
        self.time_expiry = ::std::option::Option::None;
        self.app_ids.clear();
        self.ip_addresses.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMsgSteamDatagramCertificate {
        static instance: CMsgSteamDatagramCertificate = CMsgSteamDatagramCertificate {
            key_type: ::std::option::Option::None,
            key_data: ::std::option::Option::None,
            legacy_steam_id: ::std::option::Option::None,
            legacy_identity_binary: ::protobuf::MessageField::none(),
            identity_string: ::std::option::Option::None,
            gameserver_datacenter_ids: ::std::vec::Vec::new(),
            time_created: ::std::option::Option::None,
            time_expiry: ::std::option::Option::None,
            app_ids: ::std::vec::Vec::new(),
            ip_addresses: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

/// Nested message and enums of message `CMsgSteamDatagramCertificate`
pub mod cmsg_steam_datagram_certificate {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:CMsgSteamDatagramCertificate.EKeyType)
    pub enum EKeyType {
        // @@protoc_insertion_point(enum_value:CMsgSteamDatagramCertificate.EKeyType.INVALID)
        INVALID = 0,
        // @@protoc_insertion_point(enum_value:CMsgSteamDatagramCertificate.EKeyType.ED25519)
        ED25519 = 1,
    }

    impl ::protobuf::Enum for EKeyType {
        const NAME: &'static str = "EKeyType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<EKeyType> {
            match value {
                0 => ::std::option::Option::Some(EKeyType::INVALID),
                1 => ::std::option::Option::Some(EKeyType::ED25519),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<EKeyType> {
            match str {
                "INVALID" => ::std::option::Option::Some(EKeyType::INVALID),
                "ED25519" => ::std::option::Option::Some(EKeyType::ED25519),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [EKeyType] = &[
            EKeyType::INVALID,
            EKeyType::ED25519,
        ];
    }

    impl ::std::default::Default for EKeyType {
        fn default() -> Self {
            EKeyType::INVALID
        }
    }

}

// @@protoc_insertion_point(message:CMsgSteamDatagramCertificateSigned)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMsgSteamDatagramCertificateSigned {
    // message fields
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificateSigned.cert)
    pub cert: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificateSigned.ca_key_id)
    pub ca_key_id: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificateSigned.ca_signature)
    pub ca_signature: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificateSigned.private_key_data)
    pub private_key_data: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:CMsgSteamDatagramCertificateSigned.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMsgSteamDatagramCertificateSigned {
    fn default() -> &'a CMsgSteamDatagramCertificateSigned {
        <CMsgSteamDatagramCertificateSigned as ::protobuf::Message>::default_instance()
    }
}

impl CMsgSteamDatagramCertificateSigned {
    pub fn new() -> CMsgSteamDatagramCertificateSigned {
        ::std::default::Default::default()
    }

    // optional bytes cert = 4;

    pub fn cert(&self) -> &[u8] {
        match self.cert.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_cert(&mut self) {
        self.cert = ::std::option::Option::None;
    }

    pub fn has_cert(&self) -> bool {
        self.cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: ::std::vec::Vec<u8>) {
        self.cert = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cert.is_none() {
            self.cert = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert(&mut self) -> ::std::vec::Vec<u8> {
        self.cert.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional fixed64 ca_key_id = 5;

    pub fn ca_key_id(&self) -> u64 {
        self.ca_key_id.unwrap_or(0)
    }

    pub fn clear_ca_key_id(&mut self) {
        self.ca_key_id = ::std::option::Option::None;
    }

    pub fn has_ca_key_id(&self) -> bool {
        self.ca_key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ca_key_id(&mut self, v: u64) {
        self.ca_key_id = ::std::option::Option::Some(v);
    }

    // optional bytes ca_signature = 6;

    pub fn ca_signature(&self) -> &[u8] {
        match self.ca_signature.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_ca_signature(&mut self) {
        self.ca_signature = ::std::option::Option::None;
    }

    pub fn has_ca_signature(&self) -> bool {
        self.ca_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ca_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.ca_signature = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ca_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ca_signature.is_none() {
            self.ca_signature = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.ca_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_ca_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.ca_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes private_key_data = 1;

    pub fn private_key_data(&self) -> &[u8] {
        match self.private_key_data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_private_key_data(&mut self) {
        self.private_key_data = ::std::option::Option::None;
    }

    pub fn has_private_key_data(&self) -> bool {
        self.private_key_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_key_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.private_key_data = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_key_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.private_key_data.is_none() {
            self.private_key_data = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.private_key_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_key_data(&mut self) -> ::std::vec::Vec<u8> {
        self.private_key_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CMsgSteamDatagramCertificateSigned {
    const NAME: &'static str = "CMsgSteamDatagramCertificateSigned";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    self.cert = ::std::option::Option::Some(is.read_bytes()?);
                },
                41 => {
                    self.ca_key_id = ::std::option::Option::Some(is.read_fixed64()?);
                },
                50 => {
                    self.ca_signature = ::std::option::Option::Some(is.read_bytes()?);
                },
                10 => {
                    self.private_key_data = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.cert.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.ca_key_id {
            my_size += 1 + 8;
        }
        if let Some(v) = self.ca_signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        if let Some(v) = self.private_key_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.cert.as_ref() {
            os.write_bytes(4, v)?;
        }
        if let Some(v) = self.ca_key_id {
            os.write_fixed64(5, v)?;
        }
        if let Some(v) = self.ca_signature.as_ref() {
            os.write_bytes(6, v)?;
        }
        if let Some(v) = self.private_key_data.as_ref() {
            os.write_bytes(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CMsgSteamDatagramCertificateSigned {
        CMsgSteamDatagramCertificateSigned::new()
    }

    fn clear(&mut self) {
        self.cert = ::std::option::Option::None;
        self.ca_key_id = ::std::option::Option::None;
        self.ca_signature = ::std::option::Option::None;
        self.private_key_data = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMsgSteamDatagramCertificateSigned {
        static instance: CMsgSteamDatagramCertificateSigned = CMsgSteamDatagramCertificateSigned {
            cert: ::std::option::Option::None,
            ca_key_id: ::std::option::Option::None,
            ca_signature: ::std::option::Option::None,
            private_key_data: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CMsgSteamDatagramCertificateRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMsgSteamDatagramCertificateRequest {
    // message fields
    // @@protoc_insertion_point(field:CMsgSteamDatagramCertificateRequest.cert)
    pub cert: ::protobuf::MessageField<CMsgSteamDatagramCertificate>,
    // special fields
    // @@protoc_insertion_point(special_field:CMsgSteamDatagramCertificateRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMsgSteamDatagramCertificateRequest {
    fn default() -> &'a CMsgSteamDatagramCertificateRequest {
        <CMsgSteamDatagramCertificateRequest as ::protobuf::Message>::default_instance()
    }
}

impl CMsgSteamDatagramCertificateRequest {
    pub fn new() -> CMsgSteamDatagramCertificateRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for CMsgSteamDatagramCertificateRequest {
    const NAME: &'static str = "CMsgSteamDatagramCertificateRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cert)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.cert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.cert.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CMsgSteamDatagramCertificateRequest {
        CMsgSteamDatagramCertificateRequest::new()
    }

    fn clear(&mut self) {
        self.cert.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMsgSteamDatagramCertificateRequest {
        static instance: CMsgSteamDatagramCertificateRequest = CMsgSteamDatagramCertificateRequest {
            cert: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}
impl crate::RpcMessage for CMsgSteamNetworkingIdentityLegacyBinary {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
impl crate::RpcMessage for CMsgSteamDatagramCertificate {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
impl crate::RpcMessage for CMsgSteamDatagramCertificateSigned {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
impl crate::RpcMessage for CMsgSteamDatagramCertificateRequest {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
