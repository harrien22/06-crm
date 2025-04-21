use anyhow::Result;
// use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .type_attribute("User", r#"#[derive(serde::Serialize, serde::Deserialize)]"#)
        .type_attribute("User", r#"#[serde(rename_all = "camelCase")]"#)
        .type_attribute("User", "#[derive(sqlx::FromRow)]")
        .type_attribute(
            "User",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .type_attribute(
            "QueryRequest",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .type_attribute(
            "RawQueryRequest",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .type_attribute(
            "TimeQuery",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .type_attribute(
            "IdQuery",
            "#[derive(derive_builder::Builder)]\n#[builder(setter(into, strip_option), default)]",
        )
        .field_attribute("User.email", r#"#[builder(setter(into))]"#)
        .field_attribute("User.name", r#"#[builder(setter(into))]"#)
        .field_attribute("RawQueryRequest.query", r#"#[builder(setter(into))]"#)
        .field_attribute(
            "TimeQuery.before",
            r#"#[builder(setter(into, strip_option))]"#,
        )
        .field_attribute(
            "TimeQuery.after",
            r#"#[builder(setter(into, strip_option))]"#,
        )
        .field_attribute(
            "QueryRequest.timestamps",
            r#"#[builder(setter(each(name="timestamp", into)))]"#,
        )
        .field_attribute(
            "QueryRequest.ids",
            r#"#[builder(setter(each(name="id", into)))]"#,
        )
        .compile_protos(
            &[
                "../protos/user-stats/messages.proto",
                "../protos/user-stats/rpc.proto",
            ],
            &["../protos"],
        )
        .unwrap();
    Ok(())
}
