// Replace this line
// use ic_cdk::api::Principal;

// With this line
use ic_principal::Principal;

// The rest of your code remains the same
use ic_cdk::api::caller;

#[ic_cdk::query]
fn whoami() -> Principal {
    caller() // This will return the caller's Principal
}