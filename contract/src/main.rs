#![no_std]
#![no_main]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::contract_api::{runtime, storage};
use casper_types::{Key, PublicKey, URef, U512};

const WATCHSTANDER_DICT: &str = "watchstander_dict";
const SUPERVISOR_DICT: &str = "supervisor_dict";
const ENTRY_DICT: &str = "entry_dict";

#[no_mangle]
pub extern "C" fn approve_supervisor() {
    let supervisor_public_key: PublicKey = runtime::get_named_arg("supervisor_public_key");
    let supervisor_dict: URef = storage::new_dictionary(SUPERVISOR_DICT).unwrap();
    storage::dictionary_put(supervisor_dict, &supervisor_public_key.to_string(), true);
}

#[no_mangle]
pub extern "C" fn approve_watchstander() {
    let supervisor_public_key: PublicKey = runtime::get_named_arg("supervisor_public_key");
    let watchstander_public_key: PublicKey = runtime::get_named_arg("watchstander_public_key");

    let supervisor_dict: URef = storage::new_dictionary(SUPERVISOR_DICT).unwrap();
    let is_approved: Option<bool> = storage::dictionary_get(supervisor_dict, &supervisor_public_key.to_string()).unwrap();

    if let Some(true) = is_approved {
        let watchstander_dict: URef = storage::new_dictionary(WATCHSTANDER_DICT).unwrap();
        storage::dictionary_put(watchstander_dict, &watchstander_public_key.to_string(), true);
    } else {
        runtime::revert(Error::SupervisorNotApproved)
    }
}

#[no_mangle]
pub extern "C" fn make_entry() {
    let supervisor_public_key: PublicKey = runtime::get_named_arg("supervisor_public_key");
    let watchstander_public_key: PublicKey = runtime::get_named_arg("watchstander_public_key");
    let date: String = runtime::get_named_arg("date");
    let time: String = runtime::get_named_arg("time");
    let entry: String = runtime::get_named_arg("entry");

    let supervisor_dict: URef = storage::new_dictionary(SUPERVISOR_DICT).unwrap();
    let is_supervisor_approved: Option<bool> = storage::dictionary_get(supervisor_dict, &supervisor_public_key.to_string()).unwrap();

    let watchstander_dict: URef = storage::new_dictionary(WATCHSTANDER_DICT).unwrap();
    let is_watchstander_approved: Option<bool> = storage::dictionary_get(watchstander_dict, &watchstander_public_key.to_string()).unwrap();

    if let (Some(true), Some(true)) = (is_supervisor_approved, is_watchstander_approved) {
        let entry_key = format!("{}-{}", date, time);
        let entry_dict: URef = storage::new_dictionary(ENTRY_DICT).unwrap();
        let mut entries: BTreeMap<String, String> = storage::dictionary_get(entry_dict, &date).unwrap_or_default();
        entries.insert(time, entry);
        storage::dictionary_put(entry_dict, &date, entries);
    } else {
        runtime::revert(Error::NotAuthorized)
    }
}

#[no_mangle]
pub extern "C" fn retrieve_entries_by_date() {
    let date: String = runtime::get_named_arg("date");

    let entry_dict: URef = storage::new_dictionary(ENTRY_DICT).unwrap();
    let entries: BTreeMap<String, String> = storage::dictionary_get(entry_dict, &date).unwrap_or_default();

    runtime::ret(entries, vec![]);
}

#[no_mangle]
pub extern "C" fn retrieve_entry_by_date_and_time() {
    let date: String = runtime::get_named_arg("date");
    let time: String = runtime::get_named_arg("time");

    let entry_dict: URef = storage::new_dictionary(ENTRY_DICT).unwrap();
    let entries: BTreeMap<String, String> = storage::dictionary_get(entry_dict, &date).unwrap_or_default();
    let entry = entries.get(&time);

    match entry {
        Some(entry) => runtime::ret(entry, vec![]),
        None => runtime::revert(Error::EntryNotFound),
    }
}