use clap::Parser;
use fdp_common::info::SystemDefinitionInfo;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Output folder path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.output.exists() {
        eprintln!(
            "Warning: Output directory '{}' already exists.",
            args.output.display()
        );
    } else {
        fs::create_dir_all(&args.output)?;
        println!("Created output directory: {}", args.output.display());
    }

    let definition = SystemDefinitionInfo::from(Vec::from([
        (
            "app_1".to_string(),
            fdp_definition::apps::app_1::get_definition(),
        ),
        (
            "app_2".to_string(),
            fdp_definition::apps::app_2::get_definition(),
        ),
    ]));

    generate_python_modules(&definition, &args.output)?;

    println!("Python module generation complete.");
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
requires-python = ">=3.7"

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
        generate_broadcasted_events_module(&app_info.broadcasted_events, &app_dir)?;
        generate_incoming_requests_module(&app_info.incoming_requests, &app_dir)?;
        generate_outgoing_responses_module(&app_info.outgoing_responses, &app_dir)?;
        generate_listened_events_module(&app_info.listened_events, &app_dir)?;
        generate_emitted_requests_module(&app_info.emitted_requests, &app_dir)?;

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

fn generate_broadcasted_events_module(
    events: &[fdp_common::info::MessageDeclarationInfo],
    app_dir: &PathBuf,
) -> std::io::Result<()> {
    let mut file = fs::File::create(app_dir.join("broadcasted_events.py"))?;
    writeln!(file, "from dataclasses import dataclass")?;
    writeln!(file, "from typing import ClassVar\n")?;
    for event in events {
        writeln!(file, "@dataclass")?;
        writeln!(file, "class {}:", event.identifier)?;
        writeln!(file, "    TOPIC: ClassVar[str] = '{}'", event.topic)?;
        writeln!(file, "    # Add other necessary attributes and methods\n")?;
    }
    Ok(())
}

fn generate_incoming_requests_module(
    requests: &[fdp_common::info::MessageDeclarationInfo],
    app_dir: &PathBuf,
) -> std::io::Result<()> {
    let mut file = fs::File::create(app_dir.join("incoming_requests.py"))?;
    writeln!(file, "from dataclasses import dataclass")?;
    writeln!(file, "from typing import ClassVar\n")?;
    for request in requests {
        writeln!(file, "@dataclass")?;
        writeln!(file, "class {}:", request.identifier)?;
        writeln!(file, "    TOPIC: ClassVar[str] = '{}'", request.topic)?;
        writeln!(file, "    # Add other necessary attributes and methods\n")?;
    }
    Ok(())
}

fn generate_outgoing_responses_module(
    responses: &[fdp_common::info::MessageDeclarationInfo],
    app_dir: &PathBuf,
) -> std::io::Result<()> {
    let mut file = fs::File::create(app_dir.join("outgoing_responses.py"))?;
    writeln!(file, "from dataclasses import dataclass")?;
    writeln!(file, "from typing import ClassVar\n")?;
    for response in responses {
        writeln!(file, "@dataclass")?;
        writeln!(file, "class {}:", response.identifier)?;
        writeln!(file, "    TOPIC: ClassVar[str] = '{}'", response.topic)?;
        writeln!(file, "    # Add other necessary attributes and methods\n")?;
    }
    Ok(())
}

fn generate_listened_events_module(
    events: &[fdp_common::info::MessageReferenceInfo],
    app_dir: &PathBuf,
) -> std::io::Result<()> {
    let mut file = fs::File::create(app_dir.join("listened_events.py"))?;
    for event in events {
        writeln!(
            file,
            "from ..{}.{} import {}",
            event.app_name, event.module, event.identifier
        )?;
    }
    Ok(())
}

fn generate_emitted_requests_module(
    requests: &[fdp_common::info::MessageReferenceInfo],
    app_dir: &PathBuf,
) -> std::io::Result<()> {
    let mut file = fs::File::create(app_dir.join("emitted_requests.py"))?;
    for request in requests {
        writeln!(
            file,
            "from ..{}.{} import {}",
            request.app_name, request.module, request.identifier
        )?;
    }
    Ok(())
}
