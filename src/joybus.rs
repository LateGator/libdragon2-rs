use crate::sys::joybus::*;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Identifier {
    #[doc = "Joybus identifier for an unknown or malfunctioning device."]
    Unknown = JOYBUS_IDENTIFIER_UNKNOWN as _,
    #[doc = "Joybus identifier for a port with no device connected."]
    None = JOYBUS_IDENTIFIER_NONE as _,
    #[doc = "Joybus identifier for the Nintendo 64 voice recognition peripheral (NUS-020).\n\nAlso known as VRU in North America and VRS in Japan."]
    N64VoiceRecognition = JOYBUS_IDENTIFIER_N64_VOICE_RECOGNITION as _,
    #[doc = "Joybus identifier for the Nintendo 64 Randnet keyboard peripheral (RND-001)."]
    N64RandNetKeyboard = JOYBUS_IDENTIFIER_N64_RANDNET_KEYBOARD as _,
    #[doc = "Joybus identifier for the unreleased 64GB Link Cable."]
    _64GBLinkCable = JOYBUS_IDENTIFIER_64GB_LINK_CABLE as _,
    #[doc = "Joybus identifier for a Game Boy Advance Link Cable (DOL-011)."]
    GBALinkCable = JOYBUS_IDENTIFIER_GBA_LINK_CABLE as _,
    #[doc = "Joybus identifier for cartridge-based real-time clock."]
    CartRTC = JOYBUS_IDENTIFIER_CART_RTC as _,
    #[doc = "Joybus identifier for cartridge-based 4Kbit EEPROM save type."]
    CartEEPROM4KBit = JOYBUS_IDENTIFIER_CART_EEPROM_4KBIT as _,
    #[doc = "Joybus identifier for cartridge-based 16Kbit EEPROM save type."]
    CartEEPROM16KBit = JOYBUS_IDENTIFIER_CART_EEPROM_16KBIT as _,
    #[doc = "Joybus identifier for a standard Nintendo 64 controller (NUS-005)."]
    N64Controller = JOYBUS_IDENTIFIER_N64_CONTROLLER as _,
    #[doc = "Joybus identifier for the Nintendo 64 mouse peripheral (NUS-017)."]
    N64Mouse = JOYBUS_IDENTIFIER_N64_MOUSE as _,
    #[doc = "Joybus identifier GameCube standard controller flag.\n\nFor GameCube platform devices, this bit is set if the device acts like a standard controller."]
    GcnController = (JOYBUS_IDENTIFIER_PLATFORM_GCN | JOYBUS_IDENTIFIER_MASK_GCN_CONTROLLER) as _,
    #[doc = "Joybus identifier GameCube rumble support flag.\n\nFor GameCube controllers, this bit is set if the controller DOES NOT support rumble functionality."]
    GcnNoRumble = (JOYBUS_IDENTIFIER_PLATFORM_GCN | JOYBUS_IDENTIFIER_MASK_GCN_NORUMBLE) as _,
    #[doc = "Joybus identifier GameCube wireless flag.\n\nFor GameCube controllers, this bit is set if the controller is a wireless controller."]
    GcnWireless = (JOYBUS_IDENTIFIER_PLATFORM_GCN | JOYBUS_IDENTIFIER_MASK_GCN_WIRELESS) as _,
}

impl Identifier {
    #[inline]
    pub fn is_gamecube(self) -> bool {
        (self as u16 & JOYBUS_IDENTIFIER_PLATFORM_GCN as u16) != 0
    }
}
