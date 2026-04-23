#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mycobot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotAngles() -> *const std::ffi::c_void;
}

#[link(name = "mycobot_interfaces__rosidl_generator_c")]
extern "C" {
    fn mycobot_interfaces__msg__MycobotAngles__init(msg: *mut MycobotAngles) -> bool;
    fn mycobot_interfaces__msg__MycobotAngles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MycobotAngles>, size: usize) -> bool;
    fn mycobot_interfaces__msg__MycobotAngles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MycobotAngles>);
    fn mycobot_interfaces__msg__MycobotAngles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MycobotAngles>, out_seq: *mut rosidl_runtime_rs::Sequence<MycobotAngles>) -> bool;
}

// Corresponds to mycobot_interfaces__msg__MycobotAngles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotAngles {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_1: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_2: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_3: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_4: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_5: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_6: f32,

}



impl Default for MycobotAngles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mycobot_interfaces__msg__MycobotAngles__init(&mut msg as *mut _) {
        panic!("Call to mycobot_interfaces__msg__MycobotAngles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MycobotAngles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotAngles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotAngles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotAngles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MycobotAngles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MycobotAngles where Self: Sized {
  const TYPE_NAME: &'static str = "mycobot_interfaces/msg/MycobotAngles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotAngles() }
  }
}


#[link(name = "mycobot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotCoords() -> *const std::ffi::c_void;
}

#[link(name = "mycobot_interfaces__rosidl_generator_c")]
extern "C" {
    fn mycobot_interfaces__msg__MycobotCoords__init(msg: *mut MycobotCoords) -> bool;
    fn mycobot_interfaces__msg__MycobotCoords__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MycobotCoords>, size: usize) -> bool;
    fn mycobot_interfaces__msg__MycobotCoords__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MycobotCoords>);
    fn mycobot_interfaces__msg__MycobotCoords__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MycobotCoords>, out_seq: *mut rosidl_runtime_rs::Sequence<MycobotCoords>) -> bool;
}

// Corresponds to mycobot_interfaces__msg__MycobotCoords
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotCoords {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: f32,

}



impl Default for MycobotCoords {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mycobot_interfaces__msg__MycobotCoords__init(&mut msg as *mut _) {
        panic!("Call to mycobot_interfaces__msg__MycobotCoords__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MycobotCoords {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotCoords__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotCoords__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotCoords__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MycobotCoords {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MycobotCoords where Self: Sized {
  const TYPE_NAME: &'static str = "mycobot_interfaces/msg/MycobotCoords";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotCoords() }
  }
}


#[link(name = "mycobot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotSetAngles() -> *const std::ffi::c_void;
}

#[link(name = "mycobot_interfaces__rosidl_generator_c")]
extern "C" {
    fn mycobot_interfaces__msg__MycobotSetAngles__init(msg: *mut MycobotSetAngles) -> bool;
    fn mycobot_interfaces__msg__MycobotSetAngles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MycobotSetAngles>, size: usize) -> bool;
    fn mycobot_interfaces__msg__MycobotSetAngles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MycobotSetAngles>);
    fn mycobot_interfaces__msg__MycobotSetAngles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MycobotSetAngles>, out_seq: *mut rosidl_runtime_rs::Sequence<MycobotSetAngles>) -> bool;
}

// Corresponds to mycobot_interfaces__msg__MycobotSetAngles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotSetAngles {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_1: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_2: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_3: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_4: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_5: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_6: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: i8,

}



impl Default for MycobotSetAngles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mycobot_interfaces__msg__MycobotSetAngles__init(&mut msg as *mut _) {
        panic!("Call to mycobot_interfaces__msg__MycobotSetAngles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MycobotSetAngles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotSetAngles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotSetAngles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotSetAngles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MycobotSetAngles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MycobotSetAngles where Self: Sized {
  const TYPE_NAME: &'static str = "mycobot_interfaces/msg/MycobotSetAngles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotSetAngles() }
  }
}


#[link(name = "mycobot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotSetCoords() -> *const std::ffi::c_void;
}

#[link(name = "mycobot_interfaces__rosidl_generator_c")]
extern "C" {
    fn mycobot_interfaces__msg__MycobotSetCoords__init(msg: *mut MycobotSetCoords) -> bool;
    fn mycobot_interfaces__msg__MycobotSetCoords__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MycobotSetCoords>, size: usize) -> bool;
    fn mycobot_interfaces__msg__MycobotSetCoords__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MycobotSetCoords>);
    fn mycobot_interfaces__msg__MycobotSetCoords__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MycobotSetCoords>, out_seq: *mut rosidl_runtime_rs::Sequence<MycobotSetCoords>) -> bool;
}

// Corresponds to mycobot_interfaces__msg__MycobotSetCoords
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotSetCoords {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub model: i8,

}



impl Default for MycobotSetCoords {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mycobot_interfaces__msg__MycobotSetCoords__init(&mut msg as *mut _) {
        panic!("Call to mycobot_interfaces__msg__MycobotSetCoords__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MycobotSetCoords {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotSetCoords__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotSetCoords__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotSetCoords__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MycobotSetCoords {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MycobotSetCoords where Self: Sized {
  const TYPE_NAME: &'static str = "mycobot_interfaces/msg/MycobotSetCoords";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotSetCoords() }
  }
}


#[link(name = "mycobot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotGripperStatus() -> *const std::ffi::c_void;
}

#[link(name = "mycobot_interfaces__rosidl_generator_c")]
extern "C" {
    fn mycobot_interfaces__msg__MycobotGripperStatus__init(msg: *mut MycobotGripperStatus) -> bool;
    fn mycobot_interfaces__msg__MycobotGripperStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MycobotGripperStatus>, size: usize) -> bool;
    fn mycobot_interfaces__msg__MycobotGripperStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MycobotGripperStatus>);
    fn mycobot_interfaces__msg__MycobotGripperStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MycobotGripperStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<MycobotGripperStatus>) -> bool;
}

// Corresponds to mycobot_interfaces__msg__MycobotGripperStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotGripperStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for MycobotGripperStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mycobot_interfaces__msg__MycobotGripperStatus__init(&mut msg as *mut _) {
        panic!("Call to mycobot_interfaces__msg__MycobotGripperStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MycobotGripperStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotGripperStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotGripperStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotGripperStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MycobotGripperStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MycobotGripperStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mycobot_interfaces/msg/MycobotGripperStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotGripperStatus() }
  }
}


#[link(name = "mycobot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotPumpStatus() -> *const std::ffi::c_void;
}

#[link(name = "mycobot_interfaces__rosidl_generator_c")]
extern "C" {
    fn mycobot_interfaces__msg__MycobotPumpStatus__init(msg: *mut MycobotPumpStatus) -> bool;
    fn mycobot_interfaces__msg__MycobotPumpStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MycobotPumpStatus>, size: usize) -> bool;
    fn mycobot_interfaces__msg__MycobotPumpStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MycobotPumpStatus>);
    fn mycobot_interfaces__msg__MycobotPumpStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MycobotPumpStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<MycobotPumpStatus>) -> bool;
}

// Corresponds to mycobot_interfaces__msg__MycobotPumpStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotPumpStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pin1: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pin2: i8,

}



impl Default for MycobotPumpStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mycobot_interfaces__msg__MycobotPumpStatus__init(&mut msg as *mut _) {
        panic!("Call to mycobot_interfaces__msg__MycobotPumpStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MycobotPumpStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotPumpStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotPumpStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mycobot_interfaces__msg__MycobotPumpStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MycobotPumpStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MycobotPumpStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mycobot_interfaces/msg/MycobotPumpStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mycobot_interfaces__msg__MycobotPumpStatus() }
  }
}


