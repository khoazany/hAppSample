#[macro_use]
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::{
    error::ZomeApiError,
    holochain_core_types::hash::HashString,
    holochain_dna::zome::entry_types::Sharing,
};

#[derive(Serialize, Deserialize)]
pub struct Task {
  text: String,
  complete: bool,
}

fn handle_create_task(text: String) -> serde_json::Value {
    let maybe_address = hdk::commit_entry("task", json!({
        "text": text,
        "complete": false
    }));
    match maybe_address {
        Ok(address) => {
            let link_result = hdk::link_entries(
                &HashString::from(hdk::AGENT_ADDRESS.to_string()),
                &address,
                "has tasks"
            );

            if link_result.is_err() {
                return json!({"link error": link_result.err().unwrap()})
            }

            json!({"address": address})
        }
        Err(hdk_error) => hdk_error.to_json(),
    }
}

define_zome! {
    entries: [
      entry!(
        name: "task",
        description: "a single task",
        sharing: Sharing::Public,
        native_type: Task,

        validation_package: || {
          hdk::ValidationPackageDefinition::Entry
        },

        validation: |task: Task, _ctx: hdk::ValidationData| {
          Ok(())
        }
      )
    ]

    genesis: || { Ok(()) }

    functions: {
      main (Public) {
        create_task: {
          inputs: |text: String|,
          outputs: |address: serde_json::Value|,
          handler: handle_create_task
        }
      }
    }
}
