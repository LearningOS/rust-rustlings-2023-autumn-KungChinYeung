

// fn main() {
//     // In tests7, we should set up an environment variable
//     // called `TEST_FOO`. Print in the standard output to let
//     // Cargo do it.
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs(); // What's the use of this timestamp here?
//     let your_command = format!(
//         "Your command here with {}, please checkout exercises/tests/build.rs",
//         timestamp
//     );
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//     println!("cargo:TEST_FOO={}", timestamp);
//     println!("cargo:{}", your_command);

//     // In tests8, we should enable "pass" feature to make the
//     // testcase return early. Fill in the command to tell
//     // Cargo about that.
//     let your_command = "Your command here, please checkout exercises/tests/build.rs";
//     println!("cargo:{}", your_command);
// }
fn main() {
}
