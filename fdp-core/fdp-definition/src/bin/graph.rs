use fdp_common::graph::FdpSystem;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let definition = fdp_definition::apps::get_definition();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    print!("🔍 Analysing Rust Manifest in : {}", manifest_dir);
    let system = FdpSystem::from(definition).unwrap();
    print!(" ✅ \n");

    let apps = fdp_definition::apps::get_definition().apps;

    let images_dir = Path::new(&manifest_dir).join("target/doc/images");
    fs::create_dir_all(&images_dir).expect("Failed to create images directory");

    let dot_content = system.to_graphviz();
    for (app_name, _) in apps {
        let dot_file_path = images_dir.join(format!("{}.dot", app_name));
        let png_file_path = images_dir.join(format!("{}.png", app_name));
        let md_file_path = Path::new(&manifest_dir)
            .join("src")
            .join("doc")
            .join(format!("{}.md", app_name));

        fs::write(&dot_file_path, &dot_content).expect("Failed to write dot file");

        let output = Command::new("dot")
            .args(&[
                "-Tpng",
                dot_file_path.to_str().unwrap(),
                "-o",
                png_file_path.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to execute dot command");

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            panic!("Graphviz command failed: {}", error_message);
        }

        // p!("Graph image generated at '{}'", png_file_path.display());

        let relative_image_path = format!(
            "../../../../../fdp-definition/target/doc/images/{}.png",
            app_name
        );
        let markdown_content = format!(
            "# {} Description\n![Graph Image]({})\n",
            app_name, relative_image_path
        );

        fs::create_dir_all(md_file_path.parent().unwrap()).expect("Failed to create doc directory");
        fs::write(&md_file_path, &markdown_content).expect("Failed to write markdown file");

        // println!("Updated documentation file at '{}'", md_file_path.display());
    }

    println!("📄 Successfully updated documentation")
}
