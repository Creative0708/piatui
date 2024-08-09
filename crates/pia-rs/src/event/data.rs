use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    time::SystemTime,
};

use serde_derive::{Deserialize, Serialize};

use crate::{util::ServerMap, ConstString, ServerCode};

use super::UnixTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataEventParam {
    account: AccountData,
    data: InnerData,
    state: VPNState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    active: bool,
    canceled: bool,
    days_remaining: u32,
    expiration_time: UnixTime,
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
    // TODO
    modern_latencies: ServerMap<u32>,
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
    /// Common name.
    pub cn: String,
    /// IP address.
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
    pub geo_located: bool,
    pub has_shadowsocks: bool,
    pub id: ServerCode,
    pub latency: u32,
    pub offline: bool,
    pub port_forward: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VPNState {
    pub available_locations: ServerMap<ServerState>,
    pub connection_state: ConnectionState,
    pub connected_server: Option<ConnectedServer>,
    pub external_ip: Option<Ipv4Addr>,
    pub external_vpn_ip: Option<Ipv4Addr>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedServer {
    common_name: String,
    ip: Option<Ipv4Addr>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]

pub enum ConnectionState {
    // https://github.com/pia-foss/desktop/blob/522751571ea7f6b1a9e3dd5cc4c70fc2fd136221/client/res/components/helpers/ConnStateHelper.qml#L47-L65
    Disconnected,
    Connecting,
    Reconnecting,
    DisconnectingToReconnect,
    Interrupted,
    Connected,
    Disconnecting,
}
