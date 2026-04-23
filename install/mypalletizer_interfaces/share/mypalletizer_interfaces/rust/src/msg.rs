#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mypalletizer_interfaces__msg__MypalAngles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MypalAngles::default())
  }
}

impl rosidl_runtime_rs::Message for MypalAngles {
  type RmwMsg = super::msg::rmw::MypalAngles;

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


// Corresponds to mypalletizer_interfaces__msg__MypalCoords

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MypalCoords::default())
  }
}

impl rosidl_runtime_rs::Message for MypalCoords {
  type RmwMsg = super::msg::rmw::MypalCoords;

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


// Corresponds to mypalletizer_interfaces__msg__MypalSetAngles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MypalSetAngles::default())
  }
}

impl rosidl_runtime_rs::Message for MypalSetAngles {
  type RmwMsg = super::msg::rmw::MypalSetAngles;

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


// Corresponds to mypalletizer_interfaces__msg__MypalSetCoords

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MypalSetCoords::default())
  }
}

impl rosidl_runtime_rs::Message for MypalSetCoords {
  type RmwMsg = super::msg::rmw::MypalSetCoords;

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


// Corresponds to mypalletizer_interfaces__msg__MypalGripperStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MypalGripperStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for MypalGripperStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MypalGripperStatus::default())
  }
}

impl rosidl_runtime_rs::Message for MypalGripperStatus {
  type RmwMsg = super::msg::rmw::MypalGripperStatus;

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


// Corresponds to mypalletizer_interfaces__msg__MypalPumpStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MypalPumpStatus::default())
  }
}

impl rosidl_runtime_rs::Message for MypalPumpStatus {
  type RmwMsg = super::msg::rmw::MypalPumpStatus;

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


