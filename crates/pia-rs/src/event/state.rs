use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::{
    lang::LanguageCode,
    util::{CountryCode, CountryMap, ServerMap},
};

use super::{
    data::ConnectionState,
    util::{Location, OptionalIpv4Addr},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaemonState {
    /// Let the client know whether we currently have an auth token; the client
    /// uses this to detect the "logged in, but API unreachable" state (where we
    /// will try to connect to the VPN server using credential auth).  The client
    /// can't access the actual auth token.
    pub has_account_token: bool,

    /// Boolean indicating whether the user wants to be connected or not.
    /// This specifically tracks the user's intent - this should _only_ ever be
    /// changed due to a user request to connect or disconnect.
    ///
    /// In general, any connection state can occur for any value of vpnEnabled.
    /// It is even possible to be "Disconnected" while "vpnEnabled == true"; this
    /// happens if a fatal error causes a reconnection to abort.  In this case
    /// we correctly have vpnEnabled=true, because the user intended to be
    /// connected, but the app cannot try to connect due to the fatal error.
    pub vpn_enabled: bool,
    /// The current actual state of the VPN connection.
    pub connection_state: ConnectionState,
    /// When in a connecting state, enabled when enough attempts have been made
    /// to trigger the 'slow' attempt intervals.  Resets to false before going
    /// to a non-connecting state, or when settings change during a series of
    /// attempts.
    pub using_slow_interval: bool,
    /// Boolean indicating whether a reconnect is needed in order to apply settings changes.
    pub needs_reconnect: bool,
    /// Total number of bytes received over the VPN.
    pub bytes_received: u64,
    /// Total number of bytes sent over the VPN.
    pub bytes_sent: u64,
    /// When DaemonSettings::portForward has been enabled, the port that was
    /// forwarded.  Positive values are a forwarded port; other values are
    /// special values from PortForwardState.
    pub forwarded_port: i32,
    /// External non-VPN IP address detected before connecting to the VPN
    pub external_ip: OptionalIpv4Addr,
    /// External VPN IP address detected after connecting
    pub external_vpn_ip: OptionalIpv4Addr,

    /// These are the transport settings that the user chose, and the settings
    /// that we actually connected with.  They are provided in the Connected
    /// state.
    ///
    /// The client mainly uses these to detect whether the chosen and actual
    /// transports are different.  If a connection is successfully made with
    /// alternate settings, the client will indicate the specific values used in
    /// the UI.
    ///
    /// chosenTransport.port will only be zero when a different protocol is used
    /// for actualTransport.  If the protocols are the same, and the default port
    /// was selected, then chosenTransport.port is set to the actual default port
    /// for the selected server (so the client can tell if it matches the actual
    /// transport).
    pub chosen_transport: Option<Transport>,
    pub actual_transport: Option<Transport>,

    /// Service locations chosen by the daemon, based on the chosen and best
    /// locations, etc.
    ///
    /// All location choices are provided for the VPN service and for the
    /// Shadowsocks service.  The logic for determining each one is different
    /// ("auto" means different things, for example), but the meaning of each
    /// field is the same ("the next location we would use", "the location we
    /// would use for auto", etc.)
    pub vpn_locations: ServiceLocations,
    pub shadowsocks_locations: ServiceLocations,

    /// Information about the current connection attempt and/or last established
    /// connection.  Includes VPN locations and proxy configuration.
    ///
    /// The validity of these data depends on the current state.  ('Valid' means
    /// the ConnectionInfo has a valid VPN location, and that the other setting
    /// information is meaningful.)
    ///
    /// (X = valid, - = not valid, ? = possibly valid)
    /// State                    | connectingConfig | connectedConfig
    /// -------------------------+------------------+-----------------
    /// Disconnected             | -                | ?
    /// Connecting               | X                | -
    /// Connected                | -                | X
    /// Interrupted              | X                | X
    /// Reconnecting             | X                | X
    /// DisconnectingToReconnect | X                | ?
    /// Disconnecting            | -                | ?
    ///
    /// The validity of 'connectedConfig' in Disconnected, Disconnecting and
    /// DisconnectingToReconnect depends on whether we had a connection prior to
    /// entering that state.
    ///
    /// Note that Interrupted and Reconnecting both only occur after a
    /// successful connection, so connectedLocation is always valid in those
    /// states and represents the last successful connection.
    pub connecting_config: ConnectionInfo,
    pub connected_config: ConnectionInfo,
    /// The next configuration we would use to connect if we connected right now.
    /// This is set based on the current settings, regions list, etc.
    pub next_config: ConnectionInfo,

    /// The specific server that we have connected to, when connected (only
    /// provided in the Connected state)
    pub connected_server: Option<ConnectedServer>,

    /// Available regions, mapped by region ID.  These are from either the      
    /// current or new regions list.  This includes both dedicated IP regions and
    /// regular regions, which are treated the same way by most logic referring
    /// to regions.
    pub available_locations: ServerMap<Location>,

    /// Metadata for all locations and countries; includes map coordinates,
    /// display texts, etc.
    pub regions_metadata: Metadata,

    /// Locations grouped by country and sorted by latency.  The locations are
    /// chosen from the active infrastructure specified by the "infrastructure"
    /// setting.
    ///
    /// This is used for display purposes - in regions lists, in the CLI
    /// "get regions", etc.  It does _not_ include dedicated IP regions, which
    /// are handled differently in display contexts (those are in
    /// dedicatedIpLocations)
    ///
    /// This is provided by the daemon to ensure that the client and daemon
    /// handle these in exactly the same way.  Although Daemon itself only
    /// technically cares about the lowest-latency location, the entire list must
    /// be sorted for display in the regions list.
    ///
    /// The countries are sorted by the lowest latency of any location in the
    /// country (which ensures that the lowest-latency location's country is
    /// first).  Ties are broken by country code.
    pub grouped_locations: Vec<CountryLocations>,

    /// Dedicated IP locations sorted by latency with the same tie-breaking logic
    /// as groupedLocations().  This is used in display contexts alongside
    /// groupedLocations(), as dedicated IP regions are displayed differently.
    pub dedicated_ip_locations: Vec<Location>,

    /// All supported ports for the OpenVpnUdp and OpenVpnTcp services in the
    /// active infrastructure (union of the supported ports among all advertised
    /// servers).  This can be derived from the regions lists above, but this
    /// derivation is relatively complex so these are stored.
    ///
    /// This is just used to define the choices presented in the "Remote Port"
    /// drop-down.
    pub openvpn_udp_port_choices: Vec<u16>,
    pub openvpn_tcp_port_choices: Vec<u16>,

    /// Per-interval bandwidth measurements while connected to the VPN.  Only a
    /// limited number of intervals are kept (new values past the limit will bump
    /// off the oldest value).  Older values are first.
    ///
    /// When not connected, this is an empty array.
    pub interval_measurements: Vec<IntervalBandwidth>,
    /// Timestamp when the VPN connection was established - ms since system
    /// startup, using a monotonic clock.  0 if we are not connected.
    ///
    /// Monotonic time is used so that changes in the wall-clock time won't
    /// affect the computed duration.  However, monotonic time usually excludes
    /// time while the system is sleeping/hibernating.  Most of the time, this
    /// will force us to reconnect anyway, but if the system sleeps for a short
    /// enough time that the connection is still alive, it is not too surprising
    /// that the connection duration would exclude the sleep time.
    pub connection_timestamp: i64,

    /// Set to true when the system goes in sleep mode.
    /// Set to false when waking up
    /// Note: Only implemented on macOS.
    pub system_sleeping: bool,

    /// These fields all indicate errors/warnings/notification conditions
    /// detected by the Daemon that can potentially be displayed in the client.
    /// The actual display semantics, including the message localization and
    /// whether the user can dismiss the condition, are handled by the client.
    ///
    /// Several of these are reported as timestamps so the client can observe
    /// when the problem recurs and re-show the notification if it was dismissed.
    /// Timestamps are handled as the number of milliseconds since 01-01-1970
    /// 00:00 UTC.  (Qt has a Date type in QML, but it's more cumbersome than a
    /// plain count for general use.)  0 indicates that the condition does not
    /// currently apply.

    /// Testing override(s) were present, but could not be loaded (invalid JSON,
    /// etc.).  This is set when the daemon activates, and it can be updated if
    /// the daemon deactivates and then reactivates.  It's a list of
    /// human-readable names for the resources that are overridden (not
    /// localized, this is intended for testing only).
    pub overrides_failed: Vec<String>,
    /// Testing override(s) are active.  Human-readable names of the overridden
    /// features; set at daemon startup, like overridesFailed.
    pub overrides_active: Vec<String>,
    /// Authorization failed in the OpenVPN connection (timestamp of failure).
    /// Note that this does not really mean that the user's credentials are
    /// incorrect, see ClientNotifications.qml.
    pub open_vpn_auth_failed: i64,
    /// Connection was lost (timestamp)
    pub connection_lost: i64,
    /// Failed to resolve the configured proxy.
    pub proxy_unreachable: i64,
    /// Killswitch rules blocking Internet access are active.  Note that this can
    /// apply in the Connecting/Connected states too, but usually shouldn't be
    /// displayed in these states.
    pub killswitch_enabled: bool,
    /// Available update version - set when the newest version advertised on the
    /// active release channel(s) is different from the daemon version; empty if
    /// no update is available or it is the same version as the daemon.  The
    /// client offers to download this version when it's set.
    /// Note that the download URI is not provided since it is not used by the
    /// client.
    pub available_version: String,
    /// Enabled if the current OS is out of support - newer updates are available
    /// but they do not support this OS version.
    pub os_unsupported: bool,
    /// When a download has been initiated, updateDownloadProgress indicates the
    /// progress (as a percentage).  -1 means no download is occurring,
    /// 0-100 indicates that a download is ongoing.  When the download completes,
    /// updateInstallerPath is set.
    pub update_download_progress: i32,
    /// The path to the installer for an update that has been downloaded.  Empty
    /// if no installer has been downloaded.
    pub update_installer_path: String,
    /// If a download attempt fails, updateDownloadFailure is set to the
    /// timestamp of the failure.  This is cleared when a new download is
    /// attempted.
    pub update_download_failure: i64,
    /// The version of the installer downloaded (when updateInstallerPath is
    /// set), being downloaded (when updateDownloadProgress is set), or that
    /// failed (when updateDownloadFailure is set)
    pub update_version: String,
    /// The TAP adapter is missing on Windows (the client offers to reinstall it)
    /// Not dismissible, so this is just a boolean flag.
    pub tap_adapter_missing: bool,
    /// The WinTUN driver is missing on Windows.  Like the TAP error, the client
    /// offers to reinstall it, and this is not dismissible.
    pub wintun_missing: bool,
    /// State of the network extension - the WFP callout on Windows, the
    /// transparent proxy system extension on Mac.  See Daemon::NetExtensionState.
    /// This extension is currently used for the split tunnel feature but may
    /// have other functionality in the future.
    /// This causes the client to try to install the driver before enabling the
    /// split tunnel setting if necessary, or show warnings if the driver is not
    /// installed and the setting is already enabled.
    pub net_extension_state: String,
    /// Result of the connection test performed after connecting to the VPN.  If
    /// the connection is not working, this will be set, and the client will show
    /// a warning.
    pub connection_problem: bool,
    /// A dedicated IP will expire soon.  When active, the number of days until
    /// the next expiration is also given.
    pub dedicated_ip_expiring: u64,
    pub dedicated_ip_days_remaining: i32,
    /// A dedicated IP has changed (as observed by the daemon when refreshing
    /// DIP info).  Cleared if the notification is dismissed.
    pub dedicated_ip_changed: u64,

    /// We failed to configure DNS on linux
    pub dns_config_failed: i64,
    /// Flag to indicate that the last time a client exited, it was an invalid exit
    /// and an message should possibly be displayed
    pub invalid_client_exit: bool,
    /// Flag to indicate that the daemon killed the last client connection.
    /// Similar to invalidClientExit, but does not trigger any client warning,
    /// since this is normally caused by the OS freezing the client process, and
    /// we expect the client process to reconnect.
    pub killed_client: bool,

    /// hnsd is failing to launch.  Set after it fails for 10 seconds, cleared
    /// when it launches successfully and runs for at least 30 seconds.
    /// (Timestamp of first warning.)
    pub hnsd_failing: i64,
    /// hnsd is failing to sync (but it is running, or at least it was at some
    /// point).  Set if it runs for 5 seconds without syncing a block, cleared
    /// once it syncs a block.  This can overlap with hnsdFailing if it also
    /// crashes or restarts after this condition occurs.
    pub hnsd_sync_failure: i64,

    /// The original gateway IP address before we activated the VPN
    pub original_gateway_ip: String,

    /// The original interface IP and network prefix before we activated the VPN
    pub original_interface_ip: String,
    pub original_interface_net_prefix: u32,
    pub original_mtu: u32,

    /// The original gateway interface before we activated the VPN
    pub original_interface: String,

    /// The original IPv6 interface IP, gateway, and MTU before we activated the VPN
    pub original_interface_ip6: String,
    pub original_gateway_ip6: String,
    pub original_mtu6: u32,

    /// The key for the primary service on macOS
    pub macos_primary_service_key: String,

    /// A multi-function value to indicate snooze state and
    /// -1 -> Snooze not active
    /// 0 -> Connection transitioning from "VPN Connected" to "VPN Disconnected" because user requested Snooze
    /// >0 -> The monotonic time when the snooze will be ending. Please note this can be in the past, and will be the case when the connection
    /// transitions from "VPN Disconnected" to "VPN Connected" once the snooze ends
    pub snooze_end_time: i64,

    /// If split tunnel is not available, this is set to a list of reasons.
    /// The reasons are listed in SettingsMessages.qml along with their UI text.
    /// (For example - "libnl_invalid" is set if the libnl libraries can't be
    /// loaded on Linux.)
    pub split_tunnel_support_errors: Vec<String>,

    /// A key component for the VPN is not available, all connections must be
    /// prevented the user should be warned and the daemon should not start in
    /// this state.
    pub vpn_support_errors: Vec<String>,

    /// On Mac/Linux, the name of the tunnel device being used.  Set during the
    /// [Still](Connecting|Reconnecting) states when known, remains set while
    /// connected.  Cleared in the Disconnected state.  In other states, the
    /// value depends on whether we had reached this phase of the last connection
    /// attempt.
    pub tunnel_device_name: String,
    pub tunnel_device_local_address: String,
    pub tunnel_device_remote_address: String,

    /// Whether WireGuard is available at all on this OS.  (False on Windows 7.)
    pub wireguard_available: bool,
    /// Whether a kernel implementation of Wireguard is available (only possible
    /// on Linux).
    pub wireguard_kernel_support: bool,
    /// The DNS servers prior to connecting
    pub existing_d_n_s_servers: Vec<u32>,

    /// Automation rules - indicates which rule has triggered, which rule
    /// currently matches, the rule that could be created for the current
    /// network, etc.

    /// If automation rules aren't available, this is set to a list of reasons.
    /// The possible reasons are a subset of those used for
    /// splitTunnelSupportErrors; the same text from SettingsMessages is used in
    /// the UI.
    pub automation_support_errors: Vec<String>,

    /// The rule that caused the last VPN connection transition, if there is one.
    /// If the last transition was manual instead (connect button, snooze, etc.),
    /// this is 'null'.
    ///
    /// This is a complete Rule with both action and condition
    pub automation_last_trigger: Option<AutomationRule>,

    /// The rule that matches the current network, even if it didn't cause a
    /// transition or the VPN connection was transitioned manually since then.
    /// Causes the "connected" indicator to appear in Settings.
    ///
    /// This is a rule that exists in DaemonSettings::automationRules.  If there
    /// is no custom rule for the current network, it can be a general rule if
    /// there is one that matches.
    ///
    /// If there is no network connection, or if the current connection does not
    /// match any general rule, it is 'null'.  (This is the case for connections
    /// of unknown type, which can't match any rule.)
    pub automation_current_match: Option<AutomationRule>,

    /// These are rule conditions for wireless networks that are currently
    /// connected.  Note that there can in principle be more than one of these
    /// (if the device has multiple Wi-Fi interfaces), and these may not be the
    /// default network (if, say, Ethernet is also connected).
    ///
    /// Conditions are populated for each connected wireless network, even if
    /// rules already exist for them.
    pub automation_current_networks: Vec<AutomationRuleCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionInfo {
    pub vpn_location: Option<Location>,
    pub vpn_location_auto: bool,
    pub method: ConnectionMethod,
    pub method_forced_by_auth: bool,
    pub dns_type: DNSType,
    pub openvpn_cipher: String,
    pub other_apps_use_vpn: bool,
    pub proxy: String,
    pub proxy_custom: String,
    pub proxy_shadowsocks: Option<Location>,
    pub proxy_shadowsocks_location_auto: bool,
    pub port_forward: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionMethod {
    OpenVPN,
    WireGuard,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DNSType {
    PIA,
    Handshake,
    Local,
    Existing,
    Custom,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    pub protocol: TransportProtocol,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransportProtocol {
    TCP,
    UDP,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceLocations {
    pub chosen_location: Option<Location>,
    pub best_location: Option<Location>,
    pub next_location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutomationRule {
    pub condition: AutomationRuleCondition,
    pub action: AutomationRuleAction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutomationRuleCondition {
    /// Rule type - determines what networks this condition matches
    /// - "openWifi" - any unencrypted Wi-Fi network
    /// - "protectedWifi" - any encrypted Wi-Fi network
    /// - "wired" - any wired network
    /// - "ssid" - Wi-Fi network with specified SSID (AutomationCondition::ssid())
    ///
    /// In some cases, the current network may be of a type that's not
    /// supported (such as mobile data, Bluetooth or USB tethering to a phone,
    /// etc.)  No network type is currently defined for these networks.
    pub rule_type: AutomationRuleConditionType,

    /// Wireless SSID - if set, only wireless networks with the given SSID will
    /// match.
    ///
    /// Only SSIDs that are printable UTF-8 or Latin-1 can be matched, other
    /// SSIDs cannot be represented as text.  We do not distinguish between
    /// UTF-8 and Latin-1 encodings of the same text.
    /// See NetworkConnection::ssid().
    pub ssid: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutomationRuleAction {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AutomationRuleConditionType {
    OpenWifi,
    ProtectedWifi,
    Wired,
    #[serde(rename = "ssid")]
    SSID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedServer {
    pub common_name: String,
    pub ip: OptionalIpv4Addr,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntervalBandwidth {
    pub received: u64,
    pub sent: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountryLocations {
    pub code: CountryCode,
    pub locations: Vec<Location>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub dynamic_roles: HashMap<String, DynamicRole>,
    pub country_displays: CountryMap<CountryDisplay>,
    pub region_displays: HashMap<String, RegionDisplay>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountryDisplay {
    pub name: String,
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRole {
    pub name: String,
    pub resource: String,
    pub win_icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegionDisplay {
    pub country: CountryCode,
    pub geo_latitude: f32,
    pub geo_longitude: f32,
    pub name: HashMap<LanguageCode, String>,
}
