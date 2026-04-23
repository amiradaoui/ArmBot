#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalAngles() -> *const std::ffi::c_void;
}

#[link(name = "mypalletizer_interfaces__rosidl_generator_c")]
extern "C" {
    fn mypalletizer_interfaces__msg__MypalAngles__init(msg: *mut MypalAngles) -> bool;
    fn mypalletizer_interfaces__msg__MypalAngles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MypalAngles>, size: usize) -> bool;
    fn mypalletizer_interfaces__msg__MypalAngles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MypalAngles>);
    fn mypalletizer_interfaces__msg__MypalAngles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MypalAngles>, out_seq: *mut rosidl_runtime_rs::Sequence<MypalAngles>) -> bool;
}

// Corresponds to mypalletizer_interfaces__msg__MypalAngles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalAngles {

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

}



impl Default for MypalAngles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mypalletizer_interfaces__msg__MypalAngles__init(&mut msg as *mut _) {
        panic!("Call to mypalletizer_interfaces__msg__MypalAngles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MypalAngles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalAngles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalAngles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalAngles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MypalAngles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MypalAngles where Self: Sized {
  const TYPE_NAME: &'static str = "mypalletizer_interfaces/msg/MypalAngles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalAngles() }
  }
}


#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalCoords() -> *const std::ffi::c_void;
}

#[link(name = "mypalletizer_interfaces__rosidl_generator_c")]
extern "C" {
    fn mypalletizer_interfaces__msg__MypalCoords__init(msg: *mut MypalCoords) -> bool;
    fn mypalletizer_interfaces__msg__MypalCoords__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MypalCoords>, size: usize) -> bool;
    fn mypalletizer_interfaces__msg__MypalCoords__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MypalCoords>);
    fn mypalletizer_interfaces__msg__MypalCoords__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MypalCoords>, out_seq: *mut rosidl_runtime_rs::Sequence<MypalCoords>) -> bool;
}

// Corresponds to mypalletizer_interfaces__msg__MypalCoords
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalCoords {

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

}



impl Default for MypalCoords {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mypalletizer_interfaces__msg__MypalCoords__init(&mut msg as *mut _) {
        panic!("Call to mypalletizer_interfaces__msg__MypalCoords__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MypalCoords {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalCoords__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalCoords__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalCoords__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MypalCoords {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MypalCoords where Self: Sized {
  const TYPE_NAME: &'static str = "mypalletizer_interfaces/msg/MypalCoords";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalCoords() }
  }
}


#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalSetAngles() -> *const std::ffi::c_void;
}

#[link(name = "mypalletizer_interfaces__rosidl_generator_c")]
extern "C" {
    fn mypalletizer_interfaces__msg__MypalSetAngles__init(msg: *mut MypalSetAngles) -> bool;
    fn mypalletizer_interfaces__msg__MypalSetAngles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MypalSetAngles>, size: usize) -> bool;
    fn mypalletizer_interfaces__msg__MypalSetAngles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MypalSetAngles>);
    fn mypalletizer_interfaces__msg__MypalSetAngles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MypalSetAngles>, out_seq: *mut rosidl_runtime_rs::Sequence<MypalSetAngles>) -> bool;
}

// Corresponds to mypalletizer_interfaces__msg__MypalSetAngles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalSetAngles {

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

    /// float32 joint_5
    /// float32 joint_6
    pub speed: i8,

}



impl Default for MypalSetAngles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mypalletizer_interfaces__msg__MypalSetAngles__init(&mut msg as *mut _) {
        panic!("Call to mypalletizer_interfaces__msg__MypalSetAngles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MypalSetAngles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalSetAngles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalSetAngles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalSetAngles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MypalSetAngles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MypalSetAngles where Self: Sized {
  const TYPE_NAME: &'static str = "mypalletizer_interfaces/msg/MypalSetAngles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalSetAngles() }
  }
}


#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalSetCoords() -> *const std::ffi::c_void;
}

#[link(name = "mypalletizer_interfaces__rosidl_generator_c")]
extern "C" {
    fn mypalletizer_interfaces__msg__MypalSetCoords__init(msg: *mut MypalSetCoords) -> bool;
    fn mypalletizer_interfaces__msg__MypalSetCoords__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MypalSetCoords>, size: usize) -> bool;
    fn mypalletizer_interfaces__msg__MypalSetCoords__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MypalSetCoords>);
    fn mypalletizer_interfaces__msg__MypalSetCoords__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MypalSetCoords>, out_seq: *mut rosidl_runtime_rs::Sequence<MypalSetCoords>) -> bool;
}

// Corresponds to mypalletizer_interfaces__msg__MypalSetCoords
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalSetCoords {

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

    /// float32 ry
    /// float32 rz
    pub speed: i8,

}



impl Default for MypalSetCoords {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mypalletizer_interfaces__msg__MypalSetCoords__init(&mut msg as *mut _) {
        panic!("Call to mypalletizer_interfaces__msg__MypalSetCoords__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MypalSetCoords {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalSetCoords__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalSetCoords__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalSetCoords__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MypalSetCoords {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MypalSetCoords where Self: Sized {
  const TYPE_NAME: &'static str = "mypalletizer_interfaces/msg/MypalSetCoords";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalSetCoords() }
  }
}


#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalGripperStatus() -> *const std::ffi::c_void;
}

#[link(name = "mypalletizer_interfaces__rosidl_generator_c")]
extern "C" {
    fn mypalletizer_interfaces__msg__MypalGripperStatus__init(msg: *mut MypalGripperStatus) -> bool;
    fn mypalletizer_interfaces__msg__MypalGripperStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MypalGripperStatus>, size: usize) -> bool;
    fn mypalletizer_interfaces__msg__MypalGripperStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MypalGripperStatus>);
    fn mypalletizer_interfaces__msg__MypalGripperStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MypalGripperStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<MypalGripperStatus>) -> bool;
}

// Corresponds to mypalletizer_interfaces__msg__MypalGripperStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalGripperStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for MypalGripperStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mypalletizer_interfaces__msg__MypalGripperStatus__init(&mut msg as *mut _) {
        panic!("Call to mypalletizer_interfaces__msg__MypalGripperStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MypalGripperStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalGripperStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalGripperStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalGripperStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MypalGripperStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MypalGripperStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mypalletizer_interfaces/msg/MypalGripperStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalGripperStatus() }
  }
}


#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalPumpStatus() -> *const std::ffi::c_void;
}

#[link(name = "mypalletizer_interfaces__rosidl_generator_c")]
extern "C" {
    fn mypalletizer_interfaces__msg__MypalPumpStatus__init(msg: *mut MypalPumpStatus) -> bool;
    fn mypalletizer_interfaces__msg__MypalPumpStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MypalPumpStatus>, size: usize) -> bool;
    fn mypalletizer_interfaces__msg__MypalPumpStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MypalPumpStatus>);
    fn mypalletizer_interfaces__msg__MypalPumpStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MypalPumpStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<MypalPumpStatus>) -> bool;
}

// Corresponds to mypalletizer_interfaces__msg__MypalPumpStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalPumpStatus {

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



impl Default for MypalPumpStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mypalletizer_interfaces__msg__MypalPumpStatus__init(&mut msg as *mut _) {
        panic!("Call to mypalletizer_interfaces__msg__MypalPumpStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MypalPumpStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalPumpStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalPumpStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mypalletizer_interfaces__msg__MypalPumpStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MypalPumpStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MypalPumpStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mypalletizer_interfaces/msg/MypalPumpStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mypalletizer_interfaces__msg__MypalPumpStatus() }
  }
}


