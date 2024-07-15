use std::{collections::HashMap, net::IpAddr, time::SystemTime};

use serde_derive::{Deserialize, Serialize};

use crate::{util::ServerMap, ConstString};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataEventParam {
    account: AccountData,
    data: InnerData,
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
pub struct InnerData {
    modern_latencies: ServerMap<u32>,
    // modernRegionMeta
    regions: Vec<ServerRegion>,
}

#[derive(Serialize, Deserialize, Debug)]
// these fields are snake case for some reason?
pub struct ServerRegion {
    // ???
    pub auto_region: bool,
    /// Country code of some kind. idk what standard this is
    pub country: String,
    /// Domain Name used for DNS, presumably.
    pub dns: String,
    // TODO: document
    pub geo: bool,
    /// Region ID.
    pub id: String,
    /// Region display name.
    pub name: String,
    // idk what these do
    pub offline: bool,
    pub port_forward: bool,
    /// Server info. Contains the IP addresses to actually connect to
    pub servers: HashMap<VPNConnectionType, ServerInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VPNConnectionType {
    IKEv2,
    // TODO: what is this?
    Meta,
    #[serde(rename = "ovpntcp")]
    OpenVPNTCP,
    #[serde(rename = "ovpnudp")]
    OpenVPNUDP,
    #[serde(rename = "wg")]
    WireGuard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    /// Canonical name? idk
    pub cn: String,
    /// IP address of the server
    pub ip: IpAddr,
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
    available_locations: ServerMap<ServerState>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerState {
    pub auto_safe: bool,
    pub dedicated_ip: Option<ConstString>,
}
