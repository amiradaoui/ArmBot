#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mecharm_interfaces__msg__MecharmAngles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecharmAngles::default())
  }
}

impl rosidl_runtime_rs::Message for MecharmAngles {
  type RmwMsg = super::msg::rmw::MecharmAngles;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_1: msg.joint_1,
        joint_2: msg.joint_2,
        joint_3: msg.joint_3,
        joint_4: msg.joint_4,
        joint_5: msg.joint_5,
        joint_6: msg.joint_6,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
      joint_5: msg.joint_5,
      joint_6: msg.joint_6,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
      joint_5: msg.joint_5,
      joint_6: msg.joint_6,
    }
  }
}


// Corresponds to mecharm_interfaces__msg__MecharmCoords

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecharmCoords::default())
  }
}

impl rosidl_runtime_rs::Message for MecharmCoords {
  type RmwMsg = super::msg::rmw::MecharmCoords;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to mecharm_interfaces__msg__MecharmSetAngles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecharmSetAngles::default())
  }
}

impl rosidl_runtime_rs::Message for MecharmSetAngles {
  type RmwMsg = super::msg::rmw::MecharmSetAngles;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_1: msg.joint_1,
        joint_2: msg.joint_2,
        joint_3: msg.joint_3,
        joint_4: msg.joint_4,
        joint_5: msg.joint_5,
        joint_6: msg.joint_6,
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      joint_1: msg.joint_1,
      joint_2: msg.joint_2,
      joint_3: msg.joint_3,
      joint_4: msg.joint_4,
      joint_5: msg.joint_5,
      joint_6: msg.joint_6,
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
      joint_5: msg.joint_5,
      joint_6: msg.joint_6,
      speed: msg.speed,
    }
  }
}


// Corresponds to mecharm_interfaces__msg__MecharmSetCoords

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecharmSetCoords::default())
  }
}

impl rosidl_runtime_rs::Message for MecharmSetCoords {
  type RmwMsg = super::msg::rmw::MecharmSetCoords;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
        speed: msg.speed,
        model: msg.model,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      speed: msg.speed,
      model: msg.model,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      speed: msg.speed,
      model: msg.model,
    }
  }
}


// Corresponds to mecharm_interfaces__msg__MecharmGripperStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecharmGripperStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for MecharmGripperStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecharmGripperStatus::default())
  }
}

impl rosidl_runtime_rs::Message for MecharmGripperStatus {
  type RmwMsg = super::msg::rmw::MecharmGripperStatus;

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


// Corresponds to mecharm_interfaces__msg__MecharmPumpStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecharmPumpStatus::default())
  }
}

impl rosidl_runtime_rs::Message for MecharmPumpStatus {
  type RmwMsg = super::msg::rmw::MecharmPumpStatus;

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


