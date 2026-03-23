#[test]
fn reference_section() {
    trycmd::TestCases::new().case("docs/book/src/reference/*.md");
}
