from .socket_client import SocketClient
from .message import MessageV1

import platform
import struct

class MSDPClient:
    def __init__(self, multicast_address="226.0.10.70", port=10000, unique_id=None, keepalive_timer=10):
        
        self.client = SocketClient(multicast_group=multicast_address, port=port)
        self.keepalive_timer = keepalive_timer
        self.system_name = platform.node()
        self.system_platform = platform.system()
        self.system_version = platform.version()

        # If unique_id is None, generate a unique ID (128bit UUID)
        if unique_id is None:
            import uuid
            unique_id = uuid.uuid4().bytes

        self.unique_id = unique_id



    def send_message(self):
        msg = MessageV1(unique_id=self.unique_id,
                        system_name=self.system_name,
                        system_platform=self.system_platform,
                        system_version=self.system_version,
                        keepalive_timer=self.keepalive_timer)
        self.client.send_message(msg.format())
        #print(msg.format())
        #print(f"Sent message: {self.system_name} {self.system_platform} {self.system_version}")

    def receive_message(self):
        data, address = self.client.receive_message()
        msg = MessageV1().parse(data)
        #print(f"Received message from {address}: {msg.system_name} {msg.system_platform} {msg.system_version}")
        return msg, address
    
    def close(self):
        self.client.close()
        
