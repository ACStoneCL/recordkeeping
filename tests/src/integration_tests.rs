use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{PublicKey, U512};

const WATCHSTANDER_CONTRACT: &str = "contract.wasm";

#[test]
fn should_approve_supervisor() {
    let mut context = TestContextBuilder::new()
        .with_public_key(PublicKey::ed25519_from([1u8; 32]), U512::from(500_000_000_000_000_000u64))
        .build();

    let session_code = Code::from(WATCHSTANDER_CONTRACT);
    let session_args = runtime_args! {
        "supervisor_public_key" => PublicKey::ed25519_from([2u8; 32]),
    };
    let session = SessionBuilder::new(session_code, session_args)
        .with_address(context.account_addr())
        .with_authorization_keys(&[context.account_addr()])
        .build();

    context.run(session);

    // Add assertions to check that the supervisor was approved.
}

#[test]
fn should_approve_watchstander() {
    // Similar to the above test, but call the `approve_watchstander` function instead.
}

#[test]
fn should_make_entry() {
    // Similar to the above tests, but call the `make_entry` function instead.
}

#[test]
fn should_retrieve_entries_by_date() {
    // Similar to the above tests, but call the `retrieve_entries_by_date` function instead.
}

#[test]
fn should_retrieve_entry_by_date_and_time() {
    // Similar to the above tests, but call the `retrieve_entry_by_date_and_time` function instead.
}