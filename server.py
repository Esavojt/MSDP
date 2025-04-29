import socket
import struct
import sys
import platform
import select
import os
import time
from msdp_protocol import MSDPClient

multicast_group = '226.0.10.70'

client = MSDPClient(multicast_group, port=10000)
time_to_send = time.time() + client.keepalive_timer
while True:
    if time.time() >= time_to_send:
        client.send_message()
        time_to_send = time.time() + client.keepalive_timer
    time.sleep(1)  # Send a message every 10 seconds

    readable, _, _ = select.select([client.client.sock], [], [], 1)
    if readable:
        msg, address = client.receive_message()
        if msg.unique_id == client.unique_id:
            print("Received our own message, ignoring.")
        else:
            print(f"Received message from {address[0]} {msg.system_name} {msg.system_platform} {msg.system_version} {msg.keepalive_timer}")

client.close()