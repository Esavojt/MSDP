import socket
import struct

class SocketClient:
    def __init__(self, multicast_group, port=10000):

        self.multicast_group = multicast_group
        self.port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, 1)
        
        server_address = ('', self.port)
        self.sock.bind(server_address)

        mreq = struct.pack("4s4s", socket.inet_aton(self.multicast_group), socket.inet_aton("0.0.0.0"))
        self.sock.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, mreq)
    
    def send_message(self, message: bytes):
        self.sock.sendto(message, (self.multicast_group, self.port))
    
    def receive_message(self):
        data, address = self.sock.recvfrom(2048)
        # Check if the message is not ours
        return data, address
    
    def close(self):
        self.sock.close()
