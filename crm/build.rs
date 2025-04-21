use anyhow::Result;
// use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

// fn main() -> Result<()> {
//     fs::create_dir_all("src/pb")?;
//     let builder = tonic_build::configure();
//     builder
//         .out_dir("src/pb")
//         .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
//         .with_field_attributes(
//             &["WelcomeRequest.content_ids"],
//             &[r#"#[builder(setter(each(name="content_id", into)))]"#],
//         )
//         .compile(
//             &["../protos/crm/messages.proto", "../protos/crm/rpc.proto"],
//             &["../protos"],
//         )
//         .unwrap();
//     Ok(())
// }

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .type_attribute(
            "WelcomeRequest",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .type_attribute(
            "RecallRequest",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .type_attribute(
            "RemindRequest",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .field_attribute(
            "WelcomeRequest.content_ids",
            r#"#[builder(setter(each(name="content_id", into)))]"#,
        )
        .compile_protos(
            &["../protos/crm/messages.proto", "../protos/crm/rpc.proto"],
            &["../protos"],
        )
        .unwrap();
    Ok(())
}
