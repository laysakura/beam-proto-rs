// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc 3.21.5
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

//! Generated file from `org/apache/beam/model/job_management/v1/beam_expansion_api.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:org.apache.beam.model.expansion.v1.ExpansionRequest)
pub struct ExpansionRequest {
    // message fields
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionRequest.components)
    pub components: ::protobuf::MessageField<super::beam_runner_api::Components>,
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionRequest.transform)
    pub transform: ::protobuf::MessageField<super::beam_runner_api::PTransform>,
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionRequest.namespace)
    pub namespace: ::std::string::String,
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionRequest.output_coder_requests)
    pub output_coder_requests: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:org.apache.beam.model.expansion.v1.ExpansionRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExpansionRequest {
    fn default() -> &'a ExpansionRequest {
        <ExpansionRequest as ::protobuf::Message>::default_instance()
    }
}

impl ExpansionRequest {
    pub fn new() -> ExpansionRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::beam_runner_api::Components>(
            "components",
            |m: &ExpansionRequest| { &m.components },
            |m: &mut ExpansionRequest| { &mut m.components },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::beam_runner_api::PTransform>(
            "transform",
            |m: &ExpansionRequest| { &m.transform },
            |m: &mut ExpansionRequest| { &mut m.transform },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "namespace",
            |m: &ExpansionRequest| { &m.namespace },
            |m: &mut ExpansionRequest| { &mut m.namespace },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "output_coder_requests",
            |m: &ExpansionRequest| { &m.output_coder_requests },
            |m: &mut ExpansionRequest| { &mut m.output_coder_requests },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExpansionRequest>(
            "ExpansionRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExpansionRequest {
    const NAME: &'static str = "ExpansionRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.components)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.transform)?;
                },
                26 => {
                    self.namespace = is.read_string()?;
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.output_coder_requests.insert(key, value);
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
        if let Some(v) = self.components.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.transform.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.namespace);
        }
        for (k, v) in &self.output_coder_requests {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.components.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.transform.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if !self.namespace.is_empty() {
            os.write_string(3, &self.namespace)?;
        }
        for (k, v) in &self.output_coder_requests {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
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

    fn new() -> ExpansionRequest {
        ExpansionRequest::new()
    }

    fn clear(&mut self) {
        self.components.clear();
        self.transform.clear();
        self.namespace.clear();
        self.output_coder_requests.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExpansionRequest {
        static instance: ::protobuf::rt::Lazy<ExpansionRequest> = ::protobuf::rt::Lazy::new();
        instance.get(ExpansionRequest::new)
    }
}

impl ::protobuf::MessageFull for ExpansionRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExpansionRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExpansionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExpansionRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:org.apache.beam.model.expansion.v1.ExpansionResponse)
pub struct ExpansionResponse {
    // message fields
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionResponse.components)
    pub components: ::protobuf::MessageField<super::beam_runner_api::Components>,
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionResponse.transform)
    pub transform: ::protobuf::MessageField<super::beam_runner_api::PTransform>,
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionResponse.requirements)
    pub requirements: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:org.apache.beam.model.expansion.v1.ExpansionResponse.error)
    pub error: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:org.apache.beam.model.expansion.v1.ExpansionResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExpansionResponse {
    fn default() -> &'a ExpansionResponse {
        <ExpansionResponse as ::protobuf::Message>::default_instance()
    }
}

impl ExpansionResponse {
    pub fn new() -> ExpansionResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::beam_runner_api::Components>(
            "components",
            |m: &ExpansionResponse| { &m.components },
            |m: &mut ExpansionResponse| { &mut m.components },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::beam_runner_api::PTransform>(
            "transform",
            |m: &ExpansionResponse| { &m.transform },
            |m: &mut ExpansionResponse| { &mut m.transform },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "requirements",
            |m: &ExpansionResponse| { &m.requirements },
            |m: &mut ExpansionResponse| { &mut m.requirements },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "error",
            |m: &ExpansionResponse| { &m.error },
            |m: &mut ExpansionResponse| { &mut m.error },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExpansionResponse>(
            "ExpansionResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExpansionResponse {
    const NAME: &'static str = "ExpansionResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.components)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.transform)?;
                },
                26 => {
                    self.requirements.push(is.read_string()?);
                },
                82 => {
                    self.error = is.read_string()?;
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
        if let Some(v) = self.components.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.transform.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.requirements {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.components.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.transform.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.requirements {
            os.write_string(3, &v)?;
        };
        if !self.error.is_empty() {
            os.write_string(10, &self.error)?;
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

    fn new() -> ExpansionResponse {
        ExpansionResponse::new()
    }

    fn clear(&mut self) {
        self.components.clear();
        self.transform.clear();
        self.requirements.clear();
        self.error.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExpansionResponse {
        static instance: ExpansionResponse = ExpansionResponse {
            components: ::protobuf::MessageField::none(),
            transform: ::protobuf::MessageField::none(),
            requirements: ::std::vec::Vec::new(),
            error: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ExpansionResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExpansionResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExpansionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExpansionResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n@org/apache/beam/model/job_management/v1/beam_expansion_api.proto\x12\
    \"org.apache.beam.model.expansion.v1\x1a7org/apache/beam/model/pipeline/\
    v1/beam_runner_api.proto\"\x98\x03\n\x10ExpansionRequest\x12M\n\ncompone\
    nts\x18\x01\x20\x01(\x0b2-.org.apache.beam.model.pipeline.v1.ComponentsR\
    \ncomponents\x12K\n\ttransform\x18\x02\x20\x01(\x0b2-.org.apache.beam.mo\
    del.pipeline.v1.PTransformR\ttransform\x12\x1c\n\tnamespace\x18\x03\x20\
    \x01(\tR\tnamespace\x12\x81\x01\n\x15output_coder_requests\x18\x04\x20\
    \x03(\x0b2M.org.apache.beam.model.expansion.v1.ExpansionRequest.OutputCo\
    derRequestsEntryR\x13outputCoderRequests\x1aF\n\x18OutputCoderRequestsEn\
    try\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value:\x028\x01\"\xe9\x01\n\x11ExpansionResponse\x12\
    M\n\ncomponents\x18\x01\x20\x01(\x0b2-.org.apache.beam.model.pipeline.v1\
    .ComponentsR\ncomponents\x12K\n\ttransform\x18\x02\x20\x01(\x0b2-.org.ap\
    ache.beam.model.pipeline.v1.PTransformR\ttransform\x12\"\n\x0crequiremen\
    ts\x18\x03\x20\x03(\tR\x0crequirements\x12\x14\n\x05error\x18\n\x20\x01(\
    \tR\x05error2\x89\x01\n\x10ExpansionService\x12u\n\x06Expand\x124.org.ap\
    ache.beam.model.expansion.v1.ExpansionRequest\x1a5.org.apache.beam.model\
    .expansion.v1.ExpansionResponseB\x86\x01\n\"org.apache.beam.model.expans\
    ion.v1B\x0cExpansionApiZRgithub.com/apache/beam/sdks/v2/go/pkg/beam/mode\
    l/jobmanagement_v1;jobmanagement_v1b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::beam_runner_api::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(ExpansionRequest::generated_message_descriptor_data());
            messages.push(ExpansionResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}