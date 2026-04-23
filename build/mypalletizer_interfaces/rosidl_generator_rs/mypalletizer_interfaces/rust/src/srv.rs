#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mypalletizer_interfaces__srv__GetAngles_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngles_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetAngles_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAngles_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetAngles_Request {
  type RmwMsg = super::srv::rmw::GetAngles_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__GetAngles_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

}



impl Default for GetAngles_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAngles_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetAngles_Response {
  type RmwMsg = super::srv::rmw::GetAngles_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_1: msg.joint_1,
        joint_2: msg.joint_2,
        joint_3: msg.joint_3,
        joint_4: msg.joint_4,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__SetAngles_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

    /// float32 joint_5
    /// float32 joint_6
    pub speed: i8,

}



impl Default for SetAngles_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetAngles_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetAngles_Request {
  type RmwMsg = super::srv::rmw::SetAngles_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_1: msg.joint_1,
        joint_2: msg.joint_2,
        joint_3: msg.joint_3,
        joint_4: msg.joint_4,
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
      speed: msg.speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
      speed: msg.speed,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__SetAngles_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAngles_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for SetAngles_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetAngles_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetAngles_Response {
  type RmwMsg = super::srv::rmw::SetAngles_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        flag: msg.flag,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      flag: msg.flag,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      flag: msg.flag,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__GetCoords_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCoords_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetCoords_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCoords_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetCoords_Request {
  type RmwMsg = super::srv::rmw::GetCoords_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__GetCoords_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

}



impl Default for GetCoords_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCoords_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetCoords_Response {
  type RmwMsg = super::srv::rmw::GetCoords_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__SetCoords_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

    /// float32 ry
    /// float32 rz
    pub speed: i8,

}



impl Default for SetCoords_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCoords_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Request {
  type RmwMsg = super::srv::rmw::SetCoords_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      speed: msg.speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      speed: msg.speed,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__SetCoords_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for SetCoords_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCoords_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Response {
  type RmwMsg = super::srv::rmw::SetCoords_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        flag: msg.flag,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      flag: msg.flag,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      flag: msg.flag,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__GripperStatus_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for GripperStatus_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GripperStatus_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GripperStatus_Request {
  type RmwMsg = super::srv::rmw::GripperStatus_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__GripperStatus_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for GripperStatus_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GripperStatus_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GripperStatus_Response {
  type RmwMsg = super::srv::rmw::GripperStatus_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        flag: msg.flag,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      flag: msg.flag,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      flag: msg.flag,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__PumpStatus_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PumpStatus_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PumpStatus_Request {
  type RmwMsg = super::srv::rmw::PumpStatus_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        pin1: msg.pin1,
        pin2: msg.pin2,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      pin1: msg.pin1,
      pin2: msg.pin2,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      pin1: msg.pin1,
      pin2: msg.pin2,
    }
  }
}


// Corresponds to mypalletizer_interfaces__srv__PumpStatus_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PumpStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub flag: bool,

}



impl Default for PumpStatus_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PumpStatus_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PumpStatus_Response {
  type RmwMsg = super::srv::rmw::PumpStatus_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        flag: msg.flag,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      flag: msg.flag,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      flag: msg.flag,
    }
  }
}






#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__GetAngles() -> *const std::ffi::c_void;
}

// Corresponds to mypalletizer_interfaces__srv__GetAngles
#[allow(missing_docs, non_camel_case_types)]
pub struct GetAngles;

impl rosidl_runtime_rs::Service for GetAngles {
    type Request = GetAngles_Request;
    type Response = GetAngles_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__GetAngles() }
    }
}




#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__SetAngles() -> *const std::ffi::c_void;
}

// Corresponds to mypalletizer_interfaces__srv__SetAngles
#[allow(missing_docs, non_camel_case_types)]
pub struct SetAngles;

impl rosidl_runtime_rs::Service for SetAngles {
    type Request = SetAngles_Request;
    type Response = SetAngles_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__SetAngles() }
    }
}




#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__GetCoords() -> *const std::ffi::c_void;
}

// Corresponds to mypalletizer_interfaces__srv__GetCoords
#[allow(missing_docs, non_camel_case_types)]
pub struct GetCoords;

impl rosidl_runtime_rs::Service for GetCoords {
    type Request = GetCoords_Request;
    type Response = GetCoords_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__GetCoords() }
    }
}




#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__SetCoords() -> *const std::ffi::c_void;
}

// Corresponds to mypalletizer_interfaces__srv__SetCoords
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCoords;

impl rosidl_runtime_rs::Service for SetCoords {
    type Request = SetCoords_Request;
    type Response = SetCoords_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__SetCoords() }
    }
}




#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__GripperStatus() -> *const std::ffi::c_void;
}

// Corresponds to mypalletizer_interfaces__srv__GripperStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperStatus;

impl rosidl_runtime_rs::Service for GripperStatus {
    type Request = GripperStatus_Request;
    type Response = GripperStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__GripperStatus() }
    }
}




#[link(name = "mypalletizer_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__PumpStatus() -> *const std::ffi::c_void;
}

// Corresponds to mypalletizer_interfaces__srv__PumpStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct PumpStatus;

impl rosidl_runtime_rs::Service for PumpStatus {
    type Request = PumpStatus_Request;
    type Response = PumpStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mypalletizer_interfaces__srv__PumpStatus() }
    }
}


