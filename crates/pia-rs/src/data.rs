use std::time::SystemTime;

use serde_derive::{Deserialize, Serialize};

use crate::{util::ServerMap, ConstString};


#[derive(Serialize, Deserialize, Debug)]
pub struct DataEventParam {
    account: AccountData,
    #[serde(rename = "data")]
    app_data: AppData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    active: bool,
    canceled: bool,
    days_remaining: u32,
    expiration_time: SystemTime,
    expire_alert: bool,
    expired: bool,
    logged_in: bool,
    needs_payment: bool,
    plan: String,
    recurring: bool,
    #[serde(rename = "renewURL")]
    renew_url: String,
    renewable: bool,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppData {
    // TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    // TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    // "actualTransport": null,
    // "automationCurrentMatch": null,
    // "automationCurrentNetworks": [],
    // "automationLastTrigger": null,
    // "automationSupportErrors": [],
    available_locations: ServerMap<ServerState>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerState {
    pub auto_safe: bool,
    pub dedicated_ip: Option<ConstString>,
}