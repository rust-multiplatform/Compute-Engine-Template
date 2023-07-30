<<<<<<< HEAD
=======
<<<<<<< HEAD
// use crate::main;

// Example test:
// #[test]
// fn check_main_exists() {
//     main();
// }
=======
>>>>>>> origin/upstream
use crate::android_main;

#[test]
#[cfg(target_os = "android")]
fn check_main_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    android_main();
}
<<<<<<< HEAD
=======
>>>>>>> upstream/main
>>>>>>> origin/upstream

// Example for UI tests:
// #[test]
// #[cfg(feature = "ui-tests")]
// fn some_ui_test() {
//     assert!(true);
// }
