#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmAngles() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__msg__MecharmAngles__init(msg: *mut MecharmAngles) -> bool;
    fn mecharm_interfaces__msg__MecharmAngles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecharmAngles>, size: usize) -> bool;
    fn mecharm_interfaces__msg__MecharmAngles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecharmAngles>);
    fn mecharm_interfaces__msg__MecharmAngles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecharmAngles>, out_seq: *mut rosidl_runtime_rs::Sequence<MecharmAngles>) -> bool;
}

// Corresponds to mecharm_interfaces__msg__MecharmAngles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmAngles {

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



impl Default for MecharmAngles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__msg__MecharmAngles__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__msg__MecharmAngles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecharmAngles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmAngles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmAngles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmAngles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecharmAngles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecharmAngles where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/msg/MecharmAngles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmAngles() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmCoords() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__msg__MecharmCoords__init(msg: *mut MecharmCoords) -> bool;
    fn mecharm_interfaces__msg__MecharmCoords__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecharmCoords>, size: usize) -> bool;
    fn mecharm_interfaces__msg__MecharmCoords__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecharmCoords>);
    fn mecharm_interfaces__msg__MecharmCoords__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecharmCoords>, out_seq: *mut rosidl_runtime_rs::Sequence<MecharmCoords>) -> bool;
}

// Corresponds to mecharm_interfaces__msg__MecharmCoords
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmCoords {

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



impl Default for MecharmCoords {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__msg__MecharmCoords__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__msg__MecharmCoords__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecharmCoords {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmCoords__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmCoords__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmCoords__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecharmCoords {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecharmCoords where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/msg/MecharmCoords";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmCoords() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmSetAngles() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__msg__MecharmSetAngles__init(msg: *mut MecharmSetAngles) -> bool;
    fn mecharm_interfaces__msg__MecharmSetAngles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecharmSetAngles>, size: usize) -> bool;
    fn mecharm_interfaces__msg__MecharmSetAngles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecharmSetAngles>);
    fn mecharm_interfaces__msg__MecharmSetAngles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecharmSetAngles>, out_seq: *mut rosidl_runtime_rs::Sequence<MecharmSetAngles>) -> bool;
}

// Corresponds to mecharm_interfaces__msg__MecharmSetAngles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmSetAngles {

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



impl Default for MecharmSetAngles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__msg__MecharmSetAngles__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__msg__MecharmSetAngles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecharmSetAngles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmSetAngles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmSetAngles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmSetAngles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecharmSetAngles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecharmSetAngles where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/msg/MecharmSetAngles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmSetAngles() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmSetCoords() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__msg__MecharmSetCoords__init(msg: *mut MecharmSetCoords) -> bool;
    fn mecharm_interfaces__msg__MecharmSetCoords__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecharmSetCoords>, size: usize) -> bool;
    fn mecharm_interfaces__msg__MecharmSetCoords__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecharmSetCoords>);
    fn mecharm_interfaces__msg__MecharmSetCoords__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecharmSetCoords>, out_seq: *mut rosidl_runtime_rs::Sequence<MecharmSetCoords>) -> bool;
}

// Corresponds to mecharm_interfaces__msg__MecharmSetCoords
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmSetCoords {

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



impl Default for MecharmSetCoords {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__msg__MecharmSetCoords__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__msg__MecharmSetCoords__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecharmSetCoords {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmSetCoords__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmSetCoords__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmSetCoords__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecharmSetCoords {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecharmSetCoords where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/msg/MecharmSetCoords";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmSetCoords() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmGripperStatus() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__msg__MecharmGripperStatus__init(msg: *mut MecharmGripperStatus) -> bool;
    fn mecharm_interfaces__msg__MecharmGripperStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecharmGripperStatus>, size: usize) -> bool;
    fn mecharm_interfaces__msg__MecharmGripperStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecharmGripperStatus>);
    fn mecharm_interfaces__msg__MecharmGripperStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecharmGripperStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<MecharmGripperStatus>) -> bool;
}

// Corresponds to mecharm_interfaces__msg__MecharmGripperStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmGripperStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for MecharmGripperStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__msg__MecharmGripperStatus__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__msg__MecharmGripperStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecharmGripperStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmGripperStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmGripperStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmGripperStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecharmGripperStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecharmGripperStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/msg/MecharmGripperStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmGripperStatus() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmPumpStatus() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__msg__MecharmPumpStatus__init(msg: *mut MecharmPumpStatus) -> bool;
    fn mecharm_interfaces__msg__MecharmPumpStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecharmPumpStatus>, size: usize) -> bool;
    fn mecharm_interfaces__msg__MecharmPumpStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecharmPumpStatus>);
    fn mecharm_interfaces__msg__MecharmPumpStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecharmPumpStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<MecharmPumpStatus>) -> bool;
}

// Corresponds to mecharm_interfaces__msg__MecharmPumpStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmPumpStatus {

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



impl Default for MecharmPumpStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__msg__MecharmPumpStatus__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__msg__MecharmPumpStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecharmPumpStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmPumpStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmPumpStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__msg__MecharmPumpStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecharmPumpStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecharmPumpStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/msg/MecharmPumpStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__msg__MecharmPumpStatus() }
  }
}


