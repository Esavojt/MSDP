from msdp_protocol import MSDPClient
import threading
import os
from client_hander import ControlServer

multicast_group = '226.0.10.70'


client = MSDPClient(multicast_group, port=10000, keepalive_timer=5)

control_server = ControlServer(("127.0.0.1", 10001), client)

threading.Thread(target=control_server.handle_connections, daemon=True).start()

def message_handler(msg, address, c : MSDPClient):
    #print(f"Received message from {address[0]}: {msg.system_name} {msg.system_platform} {msg.system_version} {msg.keepalive_timer}")
    #print(c)
    print("======================================")
    print(c.entriesJson())
    #for entry in c.entries:
    #    print(f"\nEntry: {entry.system_name=}\n\t{entry.system_platform=}\n\t{entry.system_version=}\n\t{entry.keepalive_timer=}\n\t{entry.address=}\n\t{entry.uptime=}s\n\t{entry.load=}\n")

client.add_message_handler(message_handler)

client.handle_connections()

client.close()