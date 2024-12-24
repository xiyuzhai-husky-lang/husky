use lean_helpers::lake_lean;

#[test]
fn lake_lean_works() {
    use expect_test::expect;

    // Print current working directory to help debug path issues
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    let path = std::path::Path::new("lean4/visored-pipeline-tests/VisoredPipelineTests/Basic.lean");
    assert!(path.exists());
    assert!(path.is_file());
    println!("{:?}", lake_lean(&path));
    expect![[r#"
        LeanOutput {
            stdout: "",
            stderr: "",
        }
    "#]].assert_debug_eq(&lake_lean(&path));
}
