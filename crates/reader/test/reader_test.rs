use reader::read_csv;

#[test]
fn test_read_file() {
    read_csv("test/sample_data.csv").unwrap();
}
