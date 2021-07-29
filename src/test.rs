#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

use crate::MALClient;
use tokio_test::block_on;
#[test]
fn encryption_test() {
    let client = block_on(MALClient::new("bruh", false, None));
    client.decrypt_tokens("bruh");
}
