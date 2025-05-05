import socket
import configparser
import json

with open('proxy_client.conf', 'r') as f:
    config = configparser.ConfigParser()
    config.read_file(f)

endpoints = {}

# Read the configuration file
for section in config.sections():
    endpoints[section] = {}
    for key, value in config.items(section):
        if key == "address":
            endpoints[section]["address"] = value
        elif key == "port":
            endpoints[section]["port"] = value


for endpoint in endpoints:
    address = endpoints[endpoint]["address"]
    port = endpoints[endpoint]["port"]
    sock_addr = (address, int(port))

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(sock_addr)

    print(f"Connecting to {sock_addr}...")
    try:
        response = sock.recv(2048)
        while response == 2048:
            response += sock.recv(2048)

        sock.close()


        print(f"========== {endpoint} ==========")
        data = json.loads(response.decode('utf-8'))
        for entry in data:
            print(f"Entry: {entry['system_name']}")
            print(f" ▪ System Platform: {entry['system_platform']}")
            print(f" ▪ System Version: {entry['system_version']}")
            print(f" ▪ Keepalive Timer: {entry['keepalive_timer']}")
            print(f" ▪ Address: {entry['address']}")
            print(f" ▪ Uptime: {entry['uptime']}s")
            print(f" ▪ Load: {entry['load']}")
            print("")

    except Exception as e:
        print(f"Error connecting to {sock_addr}: {e}")