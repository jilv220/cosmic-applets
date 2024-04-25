// SPDX-License-Identifier: GPL-3.0-or-later
//! # DBus interface proxy for: `com.system76.PowerDaemon`
//!
//! This code was generated by `zbus-xmlgen` `3.0.0` from DBus introspection data.
//! Source: `Interface '/com/system76/PowerDaemon' from service 'com.system76.PowerDaemon' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "com.system76.PowerDaemon",
    default_path = "/com/system76/PowerDaemon"
)]
trait PowerDaemon {
    /// Balanced method
    fn balanced(&self) -> zbus::Result<()>;

    /// Battery method
    fn battery(&self) -> zbus::Result<()>;

    /// GetChargeProfiles method
    fn get_charge_profiles(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// GetChargeThresholds method
    fn get_charge_thresholds(&self) -> zbus::Result<(u8, u8)>;

    /// GetDefaultGraphics method
    fn get_default_graphics(&self) -> zbus::Result<String>;

    /// GetExternalDisplaysRequireDGPU method
    fn get_external_displays_require_dgpu(&self) -> zbus::Result<bool>;

    /// GetGraphics method
    fn get_graphics(&self) -> zbus::Result<String>;

    /// GetGraphicsPower method
    fn get_graphics_power(&self) -> zbus::Result<bool>;

    /// GetProfile method
    fn get_profile(&self) -> zbus::Result<String>;

    /// GetSwitchable method
    fn get_switchable(&self) -> zbus::Result<bool>;

    /// Performance method
    fn performance(&self) -> zbus::Result<()>;

    /// SetChargeThresholds method
    fn set_charge_thresholds(&self, thresholds: &(u8, u8)) -> zbus::Result<()>;

    /// SetGraphics method
    fn set_graphics(&self, vendor: &str) -> zbus::Result<()>;

    /// SetGraphicsPower method
    fn set_graphics_power(&self, power: bool) -> zbus::Result<()>;

    /// HotPlugDetect signal
    #[dbus_proxy(signal)]
    fn hot_plug_detect(&self, port: u64) -> zbus::Result<()>;

    /// PowerProfileSwitch signal
    #[dbus_proxy(signal)]
    fn power_profile_switch(&self, profile: &str) -> zbus::Result<()>;
}
