from msdp_protocol import MSDPClient
import os
from http.server import HTTPServer, BaseHTTPRequestHandler
import threading

from client_hander import ControlServer

multicast_group = '226.0.10.70'

client = MSDPClient(multicast_group, port=10000, keepalive_timer=5)

control_server = ControlServer(("127.0.0.1", 10001), client)


threading.Thread(target=control_server.handle_connections, daemon=True).start()
threading.Thread(target=client.handle_connections, daemon=True).start()

class MyHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(bytes(client.entriesJson(), 'utf-8'))

httpd = HTTPServer(('0.0.0.0', 20000), MyHandler)
httpd.serve_forever()
