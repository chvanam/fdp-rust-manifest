use clap::Parser;
use fdp_common::info::{MessageDeclarationInfo, SystemDefinitionInfo};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    /// Output folder path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let start_time = std::time::Instant::now();

    if args.output.exists() {
        eprintln!(
            "Warning: Output directory '{}' already exists.",
            args.output.display()
        );
    } else {
        fs::create_dir_all(&args.output)?;
    }

    let definition = fdp_definition::apps::get_definition();

    generate_python_modules(&definition, &args.output)?;

    let duration = start_time.elapsed();
    println!(
        "🐍 Generated Pydantic definitions in {} in {:.2?}",
        args.output.display(),
        duration
    );
    Ok(())
}

fn generate_pyproject_toml(output_dir: &PathBuf, project_name: &str) -> std::io::Result<()> {
    let content = format!(
        r#"[build-system]
requires = ["setuptools>=61.0"]
build-backend = "setuptools.build_meta"

[project]
name = "{}"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "pydantic",
]

[tool.setuptools]
package-dir = {{"" = "src"}}

[tool.setuptools.packages.find]
where = ["src"]

"#,
        project_name
    );

    let mut file = fs::File::create(output_dir.join("pyproject.toml"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn generate_python_modules(
    system_info: &SystemDefinitionInfo,
    output_dir: &PathBuf,
) -> std::io::Result<()> {
    // Create the main package directory
    fs::create_dir_all(output_dir)?;

    // Create pyproject.toml
    generate_pyproject_toml(output_dir, "fdp_definition")?;

    // Create src/fdp_definition directory
    let src_dir = output_dir.join("src/fdp_definition");
    fs::create_dir_all(&src_dir)?;

    // Generate modules for each app
    for (app_name, app_info) in &system_info.apps {
        let app_dir = src_dir.join(app_name);
        fs::create_dir_all(&app_dir)?;

        // Generate modules for each message type
        let modules = [
            ("broadcasted_events", &app_info.broadcasted_events),
            ("incoming_requests", &app_info.incoming_requests),
            ("outgoing_responses", &app_info.outgoing_responses),
        ];

        for (module_name, items) in &modules {
            generate_pydantic_class(items, &app_dir, module_name)?;
        }

        generate_import_module(&app_info.listened_events, &app_dir, "listened_events")?;
        generate_import_module(&app_info.emitted_requests, &app_dir, "emitted_requests")?;

        // Generate __init__.py
        let mut init_file = fs::File::create(app_dir.join("__init__.py"))?;
        writeln!(init_file, "from . import broadcasted_events")?;
        writeln!(init_file, "from . import incoming_requests")?;
        writeln!(init_file, "from . import outgoing_responses")?;
        writeln!(init_file, "from . import listened_events")?;
        writeln!(init_file, "from . import emitted_requests")?;
    }

    // Generate __init__.py in the src directory
    let mut src_init_file = fs::File::create(src_dir.join("__init__.py"))?;
    for app_name in system_info.apps.keys() {
        writeln!(src_init_file, "from . import {}", app_name)?;
    }

    Ok(())
}

fn merge_json_schemas(schemas: &[(&str, Value)]) -> Value {
    let mut definitions = HashMap::new();
    for (name, schema) in schemas {
        definitions.insert(name.to_string(), schema.clone());
    }

    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": definitions,
    })
}

fn generate_pydantic_class(
    message_infos: &[MessageDeclarationInfo],
    output_dir: &PathBuf,
    module_name: &str,
) -> std::io::Result<()> {
    // Ensure the output directory exists
    fs::create_dir_all(output_dir)?;

    // Merge JSON schemas
    let schemas: Vec<(&str, Value)> = message_infos
        .iter()
        .map(|info| {
            (
                info.identifier.as_str(),
                serde_json::to_value(&info.schema).unwrap(),
            )
        })
        .collect();
    let merged_schema = merge_json_schemas(&schemas);

    // Save the merged schema to a file in the output directory
    let schema_file_path = output_dir.join(format!("{}.json", module_name));
    let mut file = File::create(&schema_file_path)?;
    file.write_all(serde_json::to_string_pretty(&merged_schema)?.as_bytes())?;

    // Generate Python Pydantic v2 classes
    let pydantic_file_path_temp = output_dir.join(format!("{}.py", module_name));
    let output = Command::new("datamodel-codegen")
        .args(&[
            "--input",
            schema_file_path.to_str().unwrap(),
            "--input-file-type",
            "jsonschema",
            "--output",
            pydantic_file_path_temp.to_str().unwrap(),
            "--output-model-type",
            "pydantic_v2.BaseModel",
            "--target-python-version",
            "3.12",
            "--use-union-operator",
            "--use-subclass-enum",
            // "--disable-timestamp",
            // "--disable-appending-item-suffix",
        ])
        .output()
        .expect("Failed to execute datamodel-codegen");

    if !output.status.success() {
        panic!(
            "Failed to generate Python Pydantic class: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn generate_import_module(
    items: &[fdp_common::info::MessageReferenceInfo],
    app_dir: &PathBuf,
    module_name: &str,
) -> std::io::Result<()> {
    let mut file = fs::File::create(app_dir.join(format!("{}.py", module_name)))?;
    for item in items {
        writeln!(
            file,
            "from ..{}.{} import {}",
            item.app_name, item.module, item.identifier
        )?;
    }
    Ok(())
}
