#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mycobot_pro450_interfaces__msg__MycobotAngles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MycobotAngles::default())
  }
}

impl rosidl_runtime_rs::Message for MycobotAngles {
  type RmwMsg = super::msg::rmw::MycobotAngles;

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


// Corresponds to mycobot_pro450_interfaces__msg__MycobotCoords

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MycobotCoords::default())
  }
}

impl rosidl_runtime_rs::Message for MycobotCoords {
  type RmwMsg = super::msg::rmw::MycobotCoords;

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


// Corresponds to mycobot_pro450_interfaces__msg__MycobotSetAngles

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MycobotSetAngles::default())
  }
}

impl rosidl_runtime_rs::Message for MycobotSetAngles {
  type RmwMsg = super::msg::rmw::MycobotSetAngles;

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


// Corresponds to mycobot_pro450_interfaces__msg__MycobotSetCoords

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

}



impl Default for MycobotSetCoords {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MycobotSetCoords::default())
  }
}

impl rosidl_runtime_rs::Message for MycobotSetCoords {
  type RmwMsg = super::msg::rmw::MycobotSetCoords;

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
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
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
      ry: msg.ry,
      rz: msg.rz,
      speed: msg.speed,
    }
  }
}


// Corresponds to mycobot_pro450_interfaces__msg__MycobotGripperStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MycobotGripperStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for MycobotGripperStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MycobotGripperStatus::default())
  }
}

impl rosidl_runtime_rs::Message for MycobotGripperStatus {
  type RmwMsg = super::msg::rmw::MycobotGripperStatus;

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


