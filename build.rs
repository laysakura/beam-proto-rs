#[cfg(feature = "build-proto")]
mod build_proto {
    use glob::glob;
    use protoc_rust::{Codegen, Customize};
    use std::{ffi::OsStr, fs, path::Path, path::PathBuf};

    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    enum Model {
        FnExecution,
        Interactive,
        JobManagement,
        Pipeline,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    struct ModelVersion(u8);

    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    struct ModName(String);
    impl ToString for ModName {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    struct ProtoMod {
        model: Model,
        model_version: ModelVersion,
        mod_name: ModName,

        proto_path: PathBuf,
    }

    impl<S> From<S> for Model
    where
        S: AsRef<str>,
    {
        fn from(s: S) -> Self {
            match s.as_ref() {
                "fn_execution" => Model::FnExecution,
                "interactive" => Model::Interactive,
                "job_management" => Model::JobManagement,
                "pipeline" => Model::Pipeline,
                _ => panic!("Unexpected model name `{}`", s.as_ref()),
            }
        }
    }

    impl Model {
        fn all() -> Vec<Self> {
            vec![
                Self::FnExecution,
                Self::Interactive,
                Self::JobManagement,
                Self::Pipeline,
            ]
        }

        fn mod_rs_file(&self) -> &str {
            match self {
                Model::FnExecution => "src/fn_execution.rs",
                Model::Interactive => "src/interactive.rs",
                Model::JobManagement => "src/job_management.rs",
                Model::Pipeline => "src/pipeline.rs",
            }
        }

        fn codegen_out_dir(&self) -> &str {
            match self {
                Model::FnExecution => "src/fn_execution/",
                Model::Interactive => "src/interactive/",
                Model::JobManagement => "src/job_management/",
                Model::Pipeline => "src/pipeline/",
            }
        }

        fn codegen_includes(&self) -> &[&str] {
            match self {
                Model::FnExecution => &[
                    "beam/model/pipeline/src/main/proto/",
                    "beam/model/fn-execution/src/main/proto/",
                ],
                Model::Interactive => &[
                    "beam/model/pipeline/src/main/proto/",
                    "beam/model/interactive/src/main/proto/",
                ],
                Model::JobManagement => &[
                    "beam/model/job-management/src/main/proto/",
                    "beam/model/interactive/src/main/proto/",
                    "beam/model/pipeline/src/main/proto/",
                ],
                Model::Pipeline => &["beam/model/pipeline/src/main/proto/"],
            }
        }
    }

    impl From<PathBuf> for ProtoMod {
        fn from(proto_path: PathBuf) -> Self {
            let mod_name = proto_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let mod_name = ModName(mod_name);

            let model_version = proto_path
                .parent()
                .and_then(|version_path| {
                    if let Some(("", version)) = version_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .split_once('v')
                    {
                        version.parse::<u8>().ok().map(ModelVersion)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    panic!("Proto path must have version directory: {:?}", proto_path)
                });

            let model_path = proto_path.parent().unwrap().parent().unwrap();
            let model_name = model_path.file_name().unwrap().to_string_lossy();
            let model = Model::from(model_name);

            Self {
                mod_name,
                model_version,
                model,
                proto_path,
            }
        }
    }

    fn proto_files_recursive(proto_dir: &Path) -> Vec<PathBuf> {
        glob(&format!("{}/**/*.proto", proto_dir.to_string_lossy()))
            .expect("Could not read protobuf directory")
            .filter_map(|entry| {
                let path = entry.ok()?;
                (path.is_file() && path.extension() == Some(OsStr::new("proto"))).then(|| path)
            })
            .collect()
    }

    /// This build script currently asserts all beam models are v1.
    fn assert_v1(input_proto_mods: &[ProtoMod]) {
        for p in input_proto_mods {
            assert!(
                p.model_version == ModelVersion(1),
                "This build script currently asserts all beam models are v1 but found: {:?}",
                p.proto_path
            )
        }
    }

    fn codegen(input_proto_mods: &[ProtoMod], model: &Model) {
        let inputs = input_proto_mods
            .iter()
            .filter(|p| &p.model == model)
            .map(|p| p.proto_path.as_path());

        Codegen::new()
            .out_dir(model.codegen_out_dir())
            .includes(model.codegen_includes())
            .inputs(inputs)
            .customize(Customize {
                expose_fields: Some(true),
                ..Default::default()
            })
            .run()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to convert protobuf into Rust for model `{:?}`",
                    model
                )
            });

        eprintln!(
            "Successfully converted protobufs ({:?} model) into {}",
            model,
            model.codegen_out_dir()
        )
    }

    fn write_mod_rs(input_proto_mods: &[ProtoMod], model: &Model) {
        let mod_names = input_proto_mods
            .iter()
            .filter(|p| &p.model == model)
            .map(|p| &p.mod_name);

        fs::write(
            model.mod_rs_file(),
            mod_names
                .map(|mod_name| format!("pub mod {};", mod_name.to_string()))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();

        eprintln!("Wrote {}", model.mod_rs_file())
    }

    pub fn main() {
        let in_dir = "./beam/model";

        let input_files = proto_files_recursive(Path::new(in_dir));
        let input_proto_mods = input_files
            .into_iter()
            .map(ProtoMod::from)
            .collect::<Vec<_>>();

        assert_v1(&input_proto_mods);

        for model in Model::all() {
            codegen(&input_proto_mods, &model);
            write_mod_rs(&input_proto_mods, &model)
        }
    }
}

#[cfg(feature = "build-proto")]
fn main() {
    build_proto::main()
}

#[cfg(not(feature = "build-proto"))]
fn main() {
    eprintln!("Nothing to do. Use `build-proto` feature to build .proto into .rs");
}
