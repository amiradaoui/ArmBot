import rclpy
from rclpy.node import Node
from sensor_msgs.msg import JointState

class SlowDemo(Node):
    def __init__(self):
        super().__init__('slow_demo_node')
        self.publisher_ = self.create_publisher(JointState, 'joint_states', 10)
        # 5.0 تعني حركة واحدة كل 5 ثوانٍ - هدوء تام للعرض
        self.timer = self.create_timer(5.0, self.move_robot)
        self.step = 0

    def move_robot(self):
        msg = JointState()
        msg.header.stamp = self.get_clock().now().to_msg()
        # أسماء المفاصل المخصصة لنسخة الـ M5 مع المقبض
        msg.name = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6', 
                    'gripper_left_joint', 'gripper_right_joint']
        
        if self.step == 0:
            msg.position = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
            print("الوضعية: الاستعداد")
            self.step = 1
        elif self.step == 1:
            msg.position = [0.8, -0.2, 0.2, 0.0, 0.0, 0.0, 0.02, -0.02]
            print("الوضعية: دوران وفتح المقبض")
            self.step = 2
        elif self.step == 2:
            msg.position = [0.8, -0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0]
            print("الوضعية: إغلاق المقبض (إمساك)")
            self.step = 3
        else:
            msg.position = [-0.8, 0.0, 0.0, 0.5, 0.8, 0.0, 0.0, 0.0]
            print("الوضعية: رفع ونقل")
            self.step = 0
            
        self.publisher_.publish(msg)

def main():
    rclpy.init()
    node = SlowDemo()
    print("بدأ العرض التلقائي البطيء... يرجى إغلاق نافذة الـ Sliders")
    rclpy.spin(node)
    rclpy.shutdown()

if __name__ == '__main__':
    main()
