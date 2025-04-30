from msdp_protocol import MSDPClient

multicast_group = '226.0.10.70'

client = MSDPClient(multicast_group, port=10000, keepalive_timer=5)

def message_handler(msg, address, c : MSDPClient):
    #print(f"Received message from {address[0]}: {msg.system_name} {msg.system_platform} {msg.system_version} {msg.keepalive_timer}")
    #print(c)
    print("======================================")
    for entry in c.entries:
        print(f"\nEntry: {entry.system_name=}\n\t{entry.system_platform=}\n\t{entry.system_version=}\n\t{entry.keepalive_timer=}\n\t{entry.address=}\n\t{entry.last_seen=}\n")

client.add_message_handler(message_handler)

client.run_indefinitely()

client.close()