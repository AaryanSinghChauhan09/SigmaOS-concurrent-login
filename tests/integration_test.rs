// Integration test for SigmaOS-concurrent-login
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use concurrent_login as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
