import struct
class MessageV1:
    """
    MessageV1 class represents a message in the MSDP protocol.
    It contains methods to parse and format MSDP messages.
    """

    def __init__(self, unique_id=None, system_name=None, system_platform=None, system_version=None, keepalive_timer=10):
        self.version = 1
        self.unique_id = unique_id
        self.system_name = system_name
        self.system_platform = system_platform
        self.system_version = system_version
        self.keepalive_timer = keepalive_timer
    

    def parse(self, data):
        """
        Parse the incoming data into the MessageV1 object.
        """
        self.unique_id = struct.unpack("!16s", data[0:16])[0]
        self.version = struct.unpack("!h", data[16:18])[0]

        system_name_length = struct.unpack("!h", data[18:20])[0]
        self.system_name = data[20:20 + system_name_length].decode('utf-8')

        offset = 20 + system_name_length
        system_platform_length = struct.unpack("!h", data[offset:offset + 2])[0]
        self.system_platform = data[offset + 2:offset + 2 + system_platform_length].decode('utf-8')
        
        offset += 2 + system_platform_length
        system_version_length = struct.unpack("!h", data[offset:offset + 2])[0]
        self.system_version = data[offset + 2:offset + 2 + system_version_length].decode('utf-8')
        
        offset += 2 + system_version_length
        self.keepalive_timer = struct.unpack("!h", data[offset:offset + 2])[0]

        return self
        
        

    def format(self):
        data = \
        struct.pack("!16s", self.unique_id) + struct.pack("!h", self.version) + \
        struct.pack("!h", len(self.system_name)) + self.system_name.encode('utf-8') + \
        struct.pack("!h", len(self.system_platform)) + self.system_platform.encode('utf-8') + \
        struct.pack("!h", len(self.system_version)) + self.system_version.encode('utf-8') + \
        struct.pack("!h", self.keepalive_timer)
        return data