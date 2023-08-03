use std::fs;
use std::path::Path;
use compact_sequence::processors::file_processing;

#[test]
fn test_process_directory() {
    let test_dir = "tests/test_data";
    let test_input_dir = format!("{}/test_inputs", test_dir);
    let output_dir = format!("{}_outputs", &test_input_dir);

    file_processing::process_directory(&test_input_dir).unwrap();

    for entry in fs::read_dir(output_dir.clone()).unwrap() {
        let entry = entry.unwrap();
        let input_file = entry.path();

        if let Some(file_name) = input_file.file_name() {
            let output_file = format!("{}/{}", output_dir, file_name.to_str().unwrap());
            let expected_output_file = format!(
                "{}/test_expected_outputs/{}", test_dir, file_name.to_str().unwrap()
            );
            
            println!("{:?}", &expected_output_file);
            assert!(Path::new(&output_file).exists());

            let output_content = fs::read_to_string(&output_file).unwrap();
            let expected_content = fs::read_to_string(&expected_output_file).unwrap();
            let output_content_normalized = output_content.replace("\r\n", "\n");
            let expected_content_normalized = expected_content.replace("\r\n", "\n");

            assert_eq!(output_content_normalized, expected_content_normalized);
        }
    }
}