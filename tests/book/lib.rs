#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("docs/book/src/concepts/*.md")
        .case("docs/book/src/reference/*.md");
}
