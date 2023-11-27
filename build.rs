fn main() {
    vergen::EmitBuilder::builder()
        .git_sha(true)
        .git_branch()
        .emit()
        .expect("Unable to generate build info");
}
