import rclpy
from rclpy.node import Node
from sensor_msgs.msg import JointState
import time

class RobotDemo(Node):
    def __init__(self):
        super().__init__('robot_demo_node')
        self.publisher_ = self.create_publisher(JointState, 'joint_states', 10)
        self.timer = self.create_timer(2.0, self.move_robot)
        self.state = 0

    def move_robot(self):
        msg = JointState()
        msg.header.stamp = self.get_clock().now().to_msg()
        # أسماء المفاصل (يجب أن تتطابق مع ملف URDF)
        msg.name = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6', 'gripper_left_joint']
        
        if self.state == 0:
            msg.position = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] # وضعية البداية
            self.state = 1
        elif self.state == 1:
            msg.position = [1.0, -0.5, 0.5, 0.0, 1.0, 0.0, 0.02] # حركة مع فتح القابض
            self.state = 2
        else:
            msg.position = [-1.0, 0.2, -0.2, 0.5, -0.5, 1.0, 0.0] # حركة أخرى مع إغلاق القابض
            self.state = 0
            
        self.publisher_.publish(msg)
        self.get_logger().info('Publishing robot movement...')

def main():
    rclpy.init()
    node = RobotDemo()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
