#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetAngles_Request() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__GetAngles_Request__init(msg: *mut GetAngles_Request) -> bool;
    fn mecharm_interfaces__srv__GetAngles_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAngles_Request>, size: usize) -> bool;
    fn mecharm_interfaces__srv__GetAngles_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAngles_Request>);
    fn mecharm_interfaces__srv__GetAngles_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAngles_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAngles_Request>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__GetAngles_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngles_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetAngles_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__GetAngles_Request__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__GetAngles_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAngles_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetAngles_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetAngles_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetAngles_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAngles_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAngles_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/GetAngles_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetAngles_Request() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetAngles_Response() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__GetAngles_Response__init(msg: *mut GetAngles_Response) -> bool;
    fn mecharm_interfaces__srv__GetAngles_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAngles_Response>, size: usize) -> bool;
    fn mecharm_interfaces__srv__GetAngles_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAngles_Response>);
    fn mecharm_interfaces__srv__GetAngles_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAngles_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAngles_Response>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__GetAngles_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngles_Response {

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



impl Default for GetAngles_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__GetAngles_Response__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__GetAngles_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAngles_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetAngles_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetAngles_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetAngles_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAngles_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAngles_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/GetAngles_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetAngles_Response() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetAngles_Request() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__SetAngles_Request__init(msg: *mut SetAngles_Request) -> bool;
    fn mecharm_interfaces__srv__SetAngles_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetAngles_Request>, size: usize) -> bool;
    fn mecharm_interfaces__srv__SetAngles_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetAngles_Request>);
    fn mecharm_interfaces__srv__SetAngles_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetAngles_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetAngles_Request>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__SetAngles_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAngles_Request {

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



impl Default for SetAngles_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__SetAngles_Request__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__SetAngles_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetAngles_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetAngles_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetAngles_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetAngles_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetAngles_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetAngles_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/SetAngles_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetAngles_Request() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetAngles_Response() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__SetAngles_Response__init(msg: *mut SetAngles_Response) -> bool;
    fn mecharm_interfaces__srv__SetAngles_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetAngles_Response>, size: usize) -> bool;
    fn mecharm_interfaces__srv__SetAngles_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetAngles_Response>);
    fn mecharm_interfaces__srv__SetAngles_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetAngles_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetAngles_Response>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__SetAngles_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAngles_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for SetAngles_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__SetAngles_Response__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__SetAngles_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetAngles_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetAngles_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetAngles_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetAngles_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetAngles_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetAngles_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/SetAngles_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetAngles_Response() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetCoords_Request() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__GetCoords_Request__init(msg: *mut GetCoords_Request) -> bool;
    fn mecharm_interfaces__srv__GetCoords_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCoords_Request>, size: usize) -> bool;
    fn mecharm_interfaces__srv__GetCoords_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCoords_Request>);
    fn mecharm_interfaces__srv__GetCoords_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCoords_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCoords_Request>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__GetCoords_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCoords_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetCoords_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__GetCoords_Request__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__GetCoords_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCoords_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetCoords_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetCoords_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetCoords_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCoords_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCoords_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/GetCoords_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetCoords_Request() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetCoords_Response() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__GetCoords_Response__init(msg: *mut GetCoords_Response) -> bool;
    fn mecharm_interfaces__srv__GetCoords_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCoords_Response>, size: usize) -> bool;
    fn mecharm_interfaces__srv__GetCoords_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCoords_Response>);
    fn mecharm_interfaces__srv__GetCoords_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCoords_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCoords_Response>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__GetCoords_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCoords_Response {

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



impl Default for GetCoords_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__GetCoords_Response__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__GetCoords_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCoords_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetCoords_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetCoords_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GetCoords_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCoords_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCoords_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/GetCoords_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GetCoords_Response() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetCoords_Request() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__SetCoords_Request__init(msg: *mut SetCoords_Request) -> bool;
    fn mecharm_interfaces__srv__SetCoords_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Request>, size: usize) -> bool;
    fn mecharm_interfaces__srv__SetCoords_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Request>);
    fn mecharm_interfaces__srv__SetCoords_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCoords_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Request>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__SetCoords_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Request {

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



impl Default for SetCoords_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__SetCoords_Request__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__SetCoords_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCoords_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetCoords_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetCoords_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetCoords_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCoords_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/SetCoords_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetCoords_Request() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetCoords_Response() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__SetCoords_Response__init(msg: *mut SetCoords_Response) -> bool;
    fn mecharm_interfaces__srv__SetCoords_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Response>, size: usize) -> bool;
    fn mecharm_interfaces__srv__SetCoords_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Response>);
    fn mecharm_interfaces__srv__SetCoords_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCoords_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Response>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__SetCoords_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for SetCoords_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__SetCoords_Response__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__SetCoords_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCoords_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetCoords_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetCoords_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__SetCoords_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCoords_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/SetCoords_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__SetCoords_Response() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GripperStatus_Request() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__GripperStatus_Request__init(msg: *mut GripperStatus_Request) -> bool;
    fn mecharm_interfaces__srv__GripperStatus_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperStatus_Request>, size: usize) -> bool;
    fn mecharm_interfaces__srv__GripperStatus_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperStatus_Request>);
    fn mecharm_interfaces__srv__GripperStatus_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperStatus_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperStatus_Request>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__GripperStatus_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for GripperStatus_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__GripperStatus_Request__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__GripperStatus_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperStatus_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GripperStatus_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GripperStatus_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GripperStatus_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperStatus_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperStatus_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/GripperStatus_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GripperStatus_Request() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GripperStatus_Response() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__GripperStatus_Response__init(msg: *mut GripperStatus_Response) -> bool;
    fn mecharm_interfaces__srv__GripperStatus_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperStatus_Response>, size: usize) -> bool;
    fn mecharm_interfaces__srv__GripperStatus_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperStatus_Response>);
    fn mecharm_interfaces__srv__GripperStatus_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperStatus_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperStatus_Response>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__GripperStatus_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for GripperStatus_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__GripperStatus_Response__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__GripperStatus_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperStatus_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GripperStatus_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GripperStatus_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__GripperStatus_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperStatus_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperStatus_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/GripperStatus_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__GripperStatus_Response() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__PumpStatus_Request() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__PumpStatus_Request__init(msg: *mut PumpStatus_Request) -> bool;
    fn mecharm_interfaces__srv__PumpStatus_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PumpStatus_Request>, size: usize) -> bool;
    fn mecharm_interfaces__srv__PumpStatus_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PumpStatus_Request>);
    fn mecharm_interfaces__srv__PumpStatus_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PumpStatus_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PumpStatus_Request>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__PumpStatus_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PumpStatus_Request {

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



impl Default for PumpStatus_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__PumpStatus_Request__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__PumpStatus_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PumpStatus_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__PumpStatus_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__PumpStatus_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__PumpStatus_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PumpStatus_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PumpStatus_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/PumpStatus_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__PumpStatus_Request() }
  }
}


#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__PumpStatus_Response() -> *const std::ffi::c_void;
}

#[link(name = "mecharm_interfaces__rosidl_generator_c")]
extern "C" {
    fn mecharm_interfaces__srv__PumpStatus_Response__init(msg: *mut PumpStatus_Response) -> bool;
    fn mecharm_interfaces__srv__PumpStatus_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PumpStatus_Response>, size: usize) -> bool;
    fn mecharm_interfaces__srv__PumpStatus_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PumpStatus_Response>);
    fn mecharm_interfaces__srv__PumpStatus_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PumpStatus_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PumpStatus_Response>) -> bool;
}

// Corresponds to mecharm_interfaces__srv__PumpStatus_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PumpStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for PumpStatus_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mecharm_interfaces__srv__PumpStatus_Response__init(&mut msg as *mut _) {
        panic!("Call to mecharm_interfaces__srv__PumpStatus_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PumpStatus_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__PumpStatus_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__PumpStatus_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mecharm_interfaces__srv__PumpStatus_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PumpStatus_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PumpStatus_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mecharm_interfaces/srv/PumpStatus_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mecharm_interfaces__srv__PumpStatus_Response() }
  }
}






#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__GetAngles() -> *const std::ffi::c_void;
}

// Corresponds to mecharm_interfaces__srv__GetAngles
#[allow(missing_docs, non_camel_case_types)]
pub struct GetAngles;

impl rosidl_runtime_rs::Service for GetAngles {
    type Request = GetAngles_Request;
    type Response = GetAngles_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__GetAngles() }
    }
}




#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__SetAngles() -> *const std::ffi::c_void;
}

// Corresponds to mecharm_interfaces__srv__SetAngles
#[allow(missing_docs, non_camel_case_types)]
pub struct SetAngles;

impl rosidl_runtime_rs::Service for SetAngles {
    type Request = SetAngles_Request;
    type Response = SetAngles_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__SetAngles() }
    }
}




#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__GetCoords() -> *const std::ffi::c_void;
}

// Corresponds to mecharm_interfaces__srv__GetCoords
#[allow(missing_docs, non_camel_case_types)]
pub struct GetCoords;

impl rosidl_runtime_rs::Service for GetCoords {
    type Request = GetCoords_Request;
    type Response = GetCoords_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__GetCoords() }
    }
}




#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__SetCoords() -> *const std::ffi::c_void;
}

// Corresponds to mecharm_interfaces__srv__SetCoords
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCoords;

impl rosidl_runtime_rs::Service for SetCoords {
    type Request = SetCoords_Request;
    type Response = SetCoords_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__SetCoords() }
    }
}




#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__GripperStatus() -> *const std::ffi::c_void;
}

// Corresponds to mecharm_interfaces__srv__GripperStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperStatus;

impl rosidl_runtime_rs::Service for GripperStatus {
    type Request = GripperStatus_Request;
    type Response = GripperStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__GripperStatus() }
    }
}




#[link(name = "mecharm_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__PumpStatus() -> *const std::ffi::c_void;
}

// Corresponds to mecharm_interfaces__srv__PumpStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct PumpStatus;

impl rosidl_runtime_rs::Service for PumpStatus {
    type Request = PumpStatus_Request;
    type Response = PumpStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mecharm_interfaces__srv__PumpStatus() }
    }
}


