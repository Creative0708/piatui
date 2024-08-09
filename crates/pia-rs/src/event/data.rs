use std::{collections::HashMap, net::Ipv4Addr};

use serde_derive::{Deserialize, Serialize};

use crate::{util::ServerMap, ConstString, ServerCode};

use super::{util::OptionalIpv4Addr, UnixTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataEventParam {
    pub account: AccountData,
    pub data: InnerData,
    pub state: VPNState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub active: bool,
    pub canceled: bool,
    pub days_remaining: u32,
    pub expiration_time: UnixTime,
    pub expire_alert: bool,
    pub expired: bool,
    pub logged_in: bool,
    pub needs_payment: bool,
    pub plan: String,
    pub recurring: bool,
    #[serde(rename = "renewURL")]
    pub renew_url: String,
    pub renewable: bool,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InnerData {
    // TODO
    pub modern_latencies: ServerMap<u32>,
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
    pub ip: Ipv4Addr,
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
    pub external_ip: OptionalIpv4Addr,
    pub external_vpn_ip: OptionalIpv4Addr,
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
