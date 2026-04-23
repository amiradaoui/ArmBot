import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/amira/mycobot_ros2/install/mycobot_280_rdkx5'
