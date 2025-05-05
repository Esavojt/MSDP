import socket
from msdp_protocol import MSDPClient

class ControlServer:
    def __init__(self, address, client: MSDPClient):
        self.address = address
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.bind(self.address)
        self.socket.listen(10)

        self.client = client

    def handle_connections(self):
        while True:
            try:
                conn, addr = self.socket.accept()
                conn.sendall(self.client.entriesJson().encode('utf-8'))
            except Exception as e:
                print(f"Error handling connection: {e}")
            finally:
                conn.close()

