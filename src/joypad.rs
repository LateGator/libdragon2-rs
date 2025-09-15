use crate::sys::joypad::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct Joypads(());

#[doc = "Joypad Port Numbers"]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Port {
    #[doc = " @brief Joypad Port 1"]
    _1 = joypad_port_t_JOYPAD_PORT_1 as _,
    #[doc = " @brief Joypad Port 2"]
    _2 = joypad_port_t_JOYPAD_PORT_2 as _,
    #[doc = " @brief Joypad Port 3"]
    _3 = joypad_port_t_JOYPAD_PORT_3 as _,
    #[doc = " @brief Joypad Port 4"]
    _4 = joypad_port_t_JOYPAD_PORT_4 as _,
}

#[doc = "Joypad Styles enumeration"]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Style {
    #[doc = "Unsupported Joypad Style"]
    None = joypad_style_t_JOYPAD_STYLE_NONE as _,
    #[doc = "Nintendo 64 Joypad Style.\n\nA standard N64 controller, which has an analog stick,\ndirectional pad, start button, L & R shoulder buttons,\na Z trigger, A & B face buttons, and a C-directional pad.\n\nFor convenience, the N64 style will coarsely simulate\ncertain GameCube controller inputs:\n\n* C-directional pad maps to the C-stick.\n* L & R shoulder buttons map to the analog triggers."]
    N64 = joypad_style_t_JOYPAD_STYLE_N64 as _,
    #[doc = "GameCube Joypad Style.\n\nA standard GameCube controller, which is supported on N64\nwhen using a passive adapter to convert the plug.\n\nThe GameCube controller has more and different buttons\nthan a Nintendo 64 controller: X & Y buttons, analog\nL & R triggers, and an analog C-stick instead of buttons.\n\nFor convenience, the GameCube style will coarsely simulate\nthe C-directional pad using C-stick inputs."]
    GCN = joypad_style_t_JOYPAD_STYLE_GCN as _,
    #[doc = "Mouse Joypad Style.\n\nThe N64 Mouse peripheral is read like a controller, but\nonly has A & B buttons, and the analog stick reports\nthe relative value since it was last read."]
    Mouse = joypad_style_t_JOYPAD_STYLE_MOUSE as _,
}

#[doc = "Joypad Accessories enumeration"]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Accessory {
    #[doc = "No accessory."]
    None = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_NONE as _,
    #[doc = "Unknown or malfunctioning accessory."]
    Unknown = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_UNKNOWN as _,
    #[doc = "Controller Pak accessory."]
    ControllerPak = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_CONTROLLER_PAK as _,
    #[doc = "Rumble Pak accessory."]
    RumblePak = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_RUMBLE_PAK as _,
    #[doc = "Transfer Pak accessory."]
    TransferPak = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_TRANSFER_PAK as _,
    #[doc = "Bio Sensor accessory."]
    BioSensor = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_BIO_SENSOR as _,
    #[doc = "Pokemon Snap Station accessory."]
    SnapStation = joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_SNAP_STATION as _,
}

bitflags::bitflags! {
    #[doc = "Joypad Buttons"]
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct Buttons: u16 {
        #[doc = "State of the A button"]
        const A        = 1<<0;
        #[doc = "State of the B button"]
        const B        = 1<<1;
        #[doc = "State of the Z button"]
        const Z        = 1<<2;
        #[doc = "State of the Start button"]
        const START    = 1<<3;
        #[doc = "State of the D-Pad Up button"]
        const D_UP     = 1<<4;
        #[doc = "State of the D-Pad Down button"]
        const D_DOWN   = 1<<5;
        #[doc = "State of the D-Pad Left button"]
        const D_LEFT   = 1<<6;
        #[doc = "State of the D-Pad Right button"]
        const D_RIGHT  = 1<<7;
        #[doc = "State of the Y button.\n\nThis input only exists on GameCube controllers."]
        const Y        = 1<<8;
        #[doc = "State of the X button.\n\nThis input only exists on GameCube controllers."]
        const X        = 1<<9;
        #[doc = "State of the digital L trigger"]
        const L        = 1<<10;
        #[doc = "State of the digital R trigger"]
        const R        = 1<<11;
        #[doc = "State of the C-Up button.\n\nFor GameCube controllers, the value will be\nemulated based on the C-Stick Y axis position."]
        const C_UP     = 1<<12;
        #[doc = "State of the C-Down button.\n\nFor GameCube controllers, the value will be\nemulated based on the C-Stick Y axis position."]
        const C_DOWN   = 1<<13;
        #[doc = "State of the C-Left button.\n\nFor GameCube controllers, the value will be\nemulated based on the C-Stick X axis position."]
        const C_LEFT   = 1<<14;
        #[doc = "State of the C-Right button.\n\nFor GameCube controllers, the value will be\nemulated based on the C-Stick X axis position."]
        const C_RIGHT  = 1<<15;
    }
}

#[doc = "Joypad Inputs Unified State Structure"]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct Inputs {
    #[doc = "Structure containing digital button inputs state."]
    pub btn: Buttons,
    #[doc = "Position of the analog joystick X axis. (-127, +127)\n\n On OEM N64 controllers with analog sticks in good condition,\n the range of this axis is roughly (-85, +85).\n\n On well-worn N64 controllers, the range may be as low as (-60, +60).\n\n On real GameCube controllers, the range is roughly (-100, +100).\n\n On startup, an N64 controller will report its current stick position\n as (0, 0). To reset the origin on an N64 controller, hold the L & R\n shoulder buttons and the start button for several seconds with the\n analog stick in a neutral position.\n\n For GameCube controllers, this value will be relative to its origin.\n The Joypad subsystem will automatically read the origins of GameCube\n controllers and account for them when resolving the analog inputs.\n To reset the origin on a GameCube controller, hold the X & Y buttons\n and the start button for several seconds with the analog inputs in\n neutral positions."]
    pub stick_x: i8,
    #[doc = "Position of the analog joystick Y axis. (-127, +127)\n\n On OEM N64 controllers with analog sticks in good condition,\n the range of this axis is roughly (-85, +85).\n\n On well-worn N64 controllers, the range may be as low as (-60, +60).\n\n On real GameCube controllers, the range is roughly (-100, +100).\n\n On startup, an N64 controller will report its current stick position\n as (0, 0). To reset the origin on an N64 controller, hold the L & R\n shoulder buttons and the start button for several seconds with the\n analog stick in a neutral position.\n\n For GameCube controllers, this value will be relative to its origin.\n The Joypad subsystem will automatically read the origins of GameCube\n controllers and account for them when resolving the analog inputs.\n To reset the origin on a GameCube controller, hold the X & Y buttons\n and the start button for several seconds with the analog inputs in\n neutral positions."]
    pub stick_y: i8,
    #[doc = "Position of the analog \"C-Stick\" X axis. (-127, +127)\n\n On real controllers, the range of this axis is roughly (-76, +76).\n\n For N64 controllers, this value will be emulated based on the\n digital C-Left and C-Right button values (-76=C-Left, +76=C-Right).\n\n For GameCube controllers, this value will be relative to its origin.\n The Joypad subsystem will automatically read the origins of GameCube\n controllers and account for them when resolving the analog inputs.\n To reset the origin on a GameCube controller, hold the X & Y buttons\n and the start button for several seconds with the analog inputs in\n neutral positions."]
    pub cstick_x: i8,
    #[doc = "Position of the analog \"C-Stick\" Y axis. (-127, +127)\n\n On real controllers, the range of this axis is roughly (-76, +76).\n\n For N64 controllers, this value will be emulated based on the\n digital C-Up and C-Down button values (-76=C-Down, +76=C-Up).\n\n For GameCube controllers, this value will be relative to its origin.\n The Joypad subsystem will automatically read the origins of GameCube\n controllers and account for them when resolving the analog inputs.\n To reset the origin on a GameCube controller, hold the X & Y buttons\n and the start button for several seconds with the analog inputs in\n neutral positions."]
    pub cstick_y: i8,
    #[doc = "Position of the analog L trigger. (0, 255)\n\n This value will be close to zero when no pressure is applied,\n and close to 200 when full pressure is applied.\n\n For N64 controllers, this value will be emulated based on the\n digital L trigger button value (0=unpressed, 200=pressed).\n\n For GameCube controllers, this value will be relative to its origin.\n The Joypad subsystem will automatically read the origins of GameCube\n controllers and account for them when resolving the analog inputs.\n To reset the origin on a GameCube controller, hold the X & Y buttons\n and the start button for several seconds with the analog inputs in\n neutral positions."]
    pub analog_l: u8,
    #[doc = "Position of the analog R trigger. (0, 255)\n\n This value will be close to zero when no pressure is applied,\n and close to 200 when full pressure is applied.\n\n For N64 controllers, this value will be emulated based on the\n digital R trigger button value (0=unpressed, 200=pressed).\n\n For GameCube controllers, this value will be relative to its origin.\n The Joypad subsystem will automatically read the origins of GameCube\n controllers and account for them when resolving the analog inputs.\n To reset the origin on a GameCube controller, hold the X & Y buttons\n and the start button for several seconds with the analog inputs in\n neutral positions."]
    pub analog_r: u8,
}
#[doc = "Joypad Axis enumeration values.\n\n These values are used to index into the `joypad_inputs_t` structure."]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Axis {
    #[doc = "Joypad stick X axis."]
    StickX = joypad_axis_t_JOYPAD_AXIS_STICK_X as _,
    #[doc = "Joypad stick Y axis."]
    StickY = joypad_axis_t_JOYPAD_AXIS_STICK_Y as _,
    #[doc = "Joypad C-stick X axis."]
    CStickX = joypad_axis_t_JOYPAD_AXIS_CSTICK_X as _,
    #[doc = "Joypad C-stick Y axis."]
    CStickY = joypad_axis_t_JOYPAD_AXIS_CSTICK_Y as _,
    #[doc = "Joypad analog L trigger axis."]
    AnalogL = joypad_axis_t_JOYPAD_AXIS_ANALOG_L as _,
    #[doc = "Joypad analog R trigger axis."]
    AnalogR = joypad_axis_t_JOYPAD_AXIS_ANALOG_R as _,
}
#[doc = "Joypad 2D axes enumeration.\n\n These values are used to select one or more 2D input axes."]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Axis2D {
    #[doc = "Analog stick 2D axes."]
    Stick = joypad_2d_t_JOYPAD_2D_STICK as _,
    #[doc = "D-Pad 2D axes."]
    DPad = joypad_2d_t_JOYPAD_2D_DPAD as _,
    #[doc = "C buttons 2D axes."]
    C = joypad_2d_t_JOYPAD_2D_C as _,
    #[doc = "Left-Hand 2D axes: Analog stick or D-Pad."]
    LH = joypad_2d_t_JOYPAD_2D_LH as _,
    #[doc = "Right-Hand 2D axes: Analog stick or C buttons."]
    RH = joypad_2d_t_JOYPAD_2D_RH as _,
    #[doc = "Any 2D axes: Analog stick, D-Pad, or C buttons."]
    Any = joypad_2d_t_JOYPAD_2D_ANY as _,
}
#[doc = "Joypad 8-way directional enumeration"]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Axis8Way {
    #[doc = "8-way no direction."]
    None = joypad_8way_t_JOYPAD_8WAY_NONE as _,
    #[doc = "8-way right direction."]
    Right = joypad_8way_t_JOYPAD_8WAY_RIGHT as _,
    #[doc = "8-way up-right direction."]
    UpRight = joypad_8way_t_JOYPAD_8WAY_UP_RIGHT as _,
    #[doc = "8-way up direction."]
    Up = joypad_8way_t_JOYPAD_8WAY_UP as _,
    #[doc = "8-way up-left direction."]
    UpLeft = joypad_8way_t_JOYPAD_8WAY_UP_LEFT as _,
    #[doc = "8-way left direction."]
    Left = joypad_8way_t_JOYPAD_8WAY_LEFT as _,
    #[doc = "8-way down-left direction."]
    DownLeft = joypad_8way_t_JOYPAD_8WAY_DOWN_LEFT as _,
    #[doc = "8-way down direction."]
    Down = joypad_8way_t_JOYPAD_8WAY_DOWN as _,
    #[doc = "8-way down-right direction."]
    DownRight = joypad_8way_t_JOYPAD_8WAY_DOWN_RIGHT as _,
}

static mut JOYPADS_INIT: bool = false;

#[doc = "Initialize the Joypad subsystem.\n\n Starts reading Joypads during VI interrupt."]
#[inline]
pub fn init() -> Joypads {
    unsafe {
        assert_eq!((&raw mut JOYPADS_INIT).read_volatile(), false);
        (&raw mut JOYPADS_INIT).write_volatile(true);
        joypad_init();
    }
    Joypads(())
}

impl Drop for Joypads {
    #[doc = "Close the Joypad subsystem.\n\n Stops reading Joypads during VI interrupt."]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            joypad_close();
            (&raw mut JOYPADS_INIT).write_volatile(false);
        }
    }
}

impl Joypads {
    #[doc = "Fetch the current Joypad input state.\n\n This function must be called once per frame, or any time after the\n Joypads have been read. After calling this function, you can read\n the Joypad state using the following functions:\n\n * `joypad_get_inputs`\n * `joypad_get_buttons`\n * `joypad_get_buttons_pressed`\n * `joypad_get_buttons_released`\n * `joypad_get_buttons_held`\n * `joypad_get_direction`\n * `joypad_get_axis_pressed`\n * `joypad_get_axis_released`\n * `joypad_get_axis_held`\n\n This function is very fast. In fact, joypads are read in background\n asynchronously under interrupt, so this function just synchronizes the\n internal state."]
    #[inline]
    pub fn poll(&self) {
        unsafe {
            joypad_poll();
        }
    }
    #[doc = "Whether a Joybus device is plugged in to a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @retval true A Joybus device is connected to the Joypad port.\n @retval false Nothing is connected to the Joypad port."]
    #[inline]
    pub fn is_connected(&self, port: Port) -> bool {
        unsafe { joypad_is_connected(port as _) }
    }
    #[doc = "Get the Joybus device identifier for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joybus device identifier (`joybus_identifier_t`)"]
    #[inline]
    pub fn identifier(&self, port: Port) -> crate::joybus::Identifier {
        unsafe { core::mem::transmute(joypad_get_identifier(port as _)) }
    }
    #[doc = "Get the Joypad style for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad style enumeration value (`joypad_style_t`)"]
    pub fn style(&self, port: Port) -> Style {
        unsafe { core::mem::transmute(joypad_get_style(port as _) as u8) }
    }
    #[doc = "Get the Joypad accessory type for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad accessory type enumeration value (`joypad_accessory_type_t`)"]
    pub fn accessory(&self, port: Port) -> Accessory {
        unsafe { core::mem::transmute(joypad_get_accessory_type(port as _) as u8) }
    }
    #[doc = "Is rumble supported for a Joypad port?\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Whether rumble is supported"]
    pub fn is_rumble_supported(&self, port: Port) -> bool {
        unsafe { joypad_get_rumble_supported(port as _) }
    }
    #[doc = "Is rumble active for a Joypad port?\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Whether rumble is active"]
    pub fn is_rumble_active(&self, port: Port) -> bool {
        unsafe { joypad_get_rumble_active(port as _) }
    }
    #[doc = "Activate or deactivate rumble on a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n @param active Whether rumble should be active"]
    pub fn set_rumble_active(&self, port: Port, active: bool) {
        unsafe { joypad_set_rumble_active(port as _, active) }
    }
    #[doc = "Get the current Joypad inputs state for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad inputs structure (`joypad_inputs_t`)"]
    pub fn inputs(&self, port: Port) -> Inputs {
        unsafe { core::mem::transmute(joypad_get_inputs(port as _)) }
    }
    #[doc = "Get the current Joypad buttons state for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad buttons structure (`joypad_buttons_t`)"]
    pub fn buttons(&self, port: Port) -> Buttons {
        unsafe { core::mem::transmute(joypad_get_buttons(port as _)) }
    }
    #[doc = "Get the Joypad buttons that were pressed since the last\n        time Joypads were read for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad buttons structure (`joypad_buttons_t`)"]
    pub fn buttons_pressed(&self, port: Port) -> Buttons {
        unsafe { core::mem::transmute(joypad_get_buttons_pressed(port as _)) }
    }
    #[doc = "Get the Joypad buttons that were released since the last\n        time Joypads were read for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad buttons structure (`joypad_buttons_t`)"]
    pub fn buttons_released(&self, port: Port) -> Buttons {
        unsafe { core::mem::transmute(joypad_get_buttons_released(port as _)) }
    }
    #[doc = "Get the Joypad buttons that are held down since the last\n        time Joypads were read for a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n\n @return Joypad buttons structure (`joypad_buttons_t`)"]
    pub fn buttons_held(&self, port: Port) -> Buttons {
        unsafe { core::mem::transmute(joypad_get_buttons_held(port as _)) }
    }
    #[doc = "Get the 8-way direction for a Joypad port's directional axes.\n\n @param port Joypad port number (`joypad_port_t`)\n @param axes 2D axes enumeration value (`joypad_2d_t`)\n @return Joypad 8-way direction enumeration value (`joypad_8way_t`)"]
    pub fn direction(&self, port: Port, axes: Axis2D) -> Axis8Way {
        unsafe { core::mem::transmute(joypad_get_direction(port as _, axes as _) as u8) }
    }
    #[doc = "Get the direction of a \"press\" of an axis on a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n @param axis Joypad axis enumeration value (`joypad_axis_t`)\n\n @retval +1 Axis is pressed in the positive direction\n @retval -1 Axis is pressed in the negative direction\n @retval  0 Axis is not pressed"]
    pub fn axis_pressed(&self, port: Port, axis: Axis) -> core::cmp::Ordering {
        unsafe { joypad_get_axis_pressed(port as _, axis as _).cmp(&0) }
    }
    #[doc = "Get the direction of a \"release\" of an axis on a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n @param axis Joypad axis enumeration value (`joypad_axis_t`)\n\n @retval +1 Axis was released in the positive direction\n @retval -1 Axis was released in the negative direction\n @retval  0 Axis is not released"]
    pub fn axis_released(&self, port: Port, axis: Axis) -> core::cmp::Ordering {
        unsafe { joypad_get_axis_released(port as _, axis as _).cmp(&0) }
    }
    #[doc = "Get the direction that an axis is held on a Joypad port.\n\n @param port Joypad port number (`joypad_port_t`)\n @param axis Joypad axis enumeration value (`joypad_axis_t`)\n\n @retval +1 Axis is being held in the positive direction\n @retval -1 Axis is being held in the negative direction\n @retval  0 Axis is not being held"]
    pub fn axis_held(&self, port: Port, axis: Axis) -> core::cmp::Ordering {
        unsafe { joypad_get_axis_held(port as _, axis as _).cmp(&0) }
    }
}

impl TryFrom<u8> for Port {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= joypad_port_t_JOYPAD_PORT_4 as _ {
            Ok(unsafe { core::mem::transmute(value) })
        } else {
            Err(u8::try_from(u32::MAX).unwrap_err())
        }
    }
}

impl TryFrom<u16> for Port {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        u8::try_from(value).and_then(Self::try_from)
    }
}

impl TryFrom<u32> for Port {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        u8::try_from(value).and_then(Self::try_from)
    }
}

impl TryFrom<usize> for Port {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u8::try_from(value).and_then(Self::try_from)
    }
}
