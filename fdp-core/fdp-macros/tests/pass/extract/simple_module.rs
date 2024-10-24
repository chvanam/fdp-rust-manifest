use fdp_common::extract::ExtractedInformation;

#[fdp::extract]
mod simple_module {}

fn main() {
    assert_eq!(
        simple_module::get_extracted_information(),
        ExtractedInformation {}
    );
}
