import socket
import json

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("localhost",10001))

data = sock.recv(1024)
while data == 1024:
    data += sock.recv(1024)

sock.close()
# Decode the received data
# Assuming the data is in JSON format
try:
    jsonData = json.loads(data.decode('utf-8'))
except json.JSONDecodeError:
    print("Failed to decode JSON data")

for entry in jsonData:
    print(f"Entry: {entry['system_name']}")
    print(f" ▪ System Platform: {entry['system_platform']}")
    print(f" ▪ System Version: {entry['system_version']}")
    print(f" ▪ Keepalive Timer: {entry['keepalive_timer']}")
    print(f" ▪ Address: {entry['address']}")
    print(f" ▪ Uptime: {entry['uptime']}s")
    print(f" ▪ Load: {entry['load']}")
    print("")