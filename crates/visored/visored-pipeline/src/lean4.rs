use lean_helpers::lake_lean;

#[test]
fn lake_lean_works() {
    use expect_test::expect;

    let path = std::path::Path::new("lean4/visored-pipeline-tests/VisoredPipelineTests/Basic.lean");
    assert!(path.exists());
    assert!(path.is_file());
    expect![[r#"
        LeanOutput {
            stdout: "",
            stderr: "",
        }
    "#]]
    .assert_debug_eq(&lake_lean(&path));
}
