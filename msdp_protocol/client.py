from .socket_client import SocketClient
from .message import MessageV1
from .handler_type import HandlerType
from .entry import Entry

import platform
import struct
import time
import select
import socket

MINIMAL_TIMER = 1

class MSDPClient:

    entries = []

    handlers = {
        HandlerType.MESSAGE: [],
        HandlerType.MESSAGE_ALL: [],
        HandlerType.MESSAGE_RAW: []
    }

    def __init__(self, multicast_address="226.0.10.70", port=10000, unique_id=None, keepalive_timer=10):
        
        self.client = SocketClient(multicast_group=multicast_address, port=port)
        self.keepalive_timer = keepalive_timer
        self.system_name = socket.getfqdn()
        self.system_platform = platform.system()
        self.system_version = platform.version()

        # If unique_id is None, generate a unique ID (128bit UUID)
        if unique_id is None:
            import uuid
            unique_id = uuid.uuid4().bytes

        self.unique_id = unique_id

        if keepalive_timer < MINIMAL_TIMER:
            print(f"Keepalive timer is too small, setting to {MINIMAL_TIMER} seconds.")
            self.keepalive_timer = MINIMAL_TIMER



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
        # Send raw message to raw message handlers
        for handler in self.handlers[HandlerType.MESSAGE_RAW]:
            handler(data, address, self)

        try:
            msg = MessageV1().parse(data)
        except struct.error as e:
            print(f"Error parsing message: {e}")
            return None, address
        
        #print(f"Received message from {address}: {msg.system_name} {msg.system_platform} {msg.system_version}")
        return msg, address
    
    def add_message_handler(self, handler, handler_type=HandlerType.MESSAGE):
        self.handlers[handler_type].append(handler)

    # Remove a message handler
    # This will remove all handlers of the specified type
    def remove_message_handler(self, handler):
        for key in list(self.handlers.keys()):
            if handler in self.handlers[key]:
                self.handlers[key].remove(handler)

    # Remove all message handlers
    def remove_all_message_handlers(self):
        for key in list(self.handlers.keys()):
            self.handlers[key] = []
        
    
    def run_indefinitely(self, verbose=False):
        print(f"Running MSDP client with unique ID: {self.unique_id.hex()}")
        time_to_send = time.time() + self.keepalive_timer
        while True:
            # Check if it's time to send a message
            if time.time() >= time_to_send:
                self.send_message()
                time_to_send = time.time() + self.keepalive_timer
            
            # Expire old entries
            self._expire_entries(verbose)

            readable, _, _ = select.select([self.client.sock], [], [], self.keepalive_timer // 2)
            if readable:
                msg, address = self.receive_message()
                for handler in self.handlers[HandlerType.MESSAGE_ALL]:
                    handler(msg, address, self)

                if msg is None:
                    continue

                if msg.unique_id == self.unique_id:
                    if verbose:
                        print(f"Received our own message, ignoring.")
                else:
                    # Check if the message is from a new entry
                    for entry in self.entries:
                        if entry.unique_id == msg.unique_id:
                            # Update the entry
                            entry.system_name = msg.system_name
                            entry.system_platform = msg.system_platform
                            entry.system_version = msg.system_version
                            entry.keepalive_timer = msg.keepalive_timer
                            entry.last_seen = time.time()
                            entry.address = address[0]
                            entry.update_expiration_time()

                            if verbose:
                                print(f"Updating entry: {entry.system_name} {entry.system_platform} {entry.system_version}")
                            break
                    else:
                        # Add a new entry
                        entry = Entry(unique_id=msg.unique_id,
                                      system_name=msg.system_name,
                                      system_platform=msg.system_platform,
                                      system_version=msg.system_version,
                                      keepalive_timer=msg.keepalive_timer,
                                      address=address[0], 
                                      last_seen=time.time())
                        if verbose:
                            print(f"Adding new entry: {entry.system_name} {entry.system_platform} {entry.system_version}")
                        self.entries.append(entry)

                    for handler in self.handlers[HandlerType.MESSAGE]:
                        handler(msg, address, self)

                    if verbose:
                        print(f"Received message from {address[0]}: {msg.system_name} {msg.system_platform} {msg.system_version} {msg.keepalive_timer}")

    def close(self):
        self.client.close()

    def _expire_entries(self, verbose):
        for entry in self.entries:
            if time.time() >= entry.expiration_time:
                self.entries.remove(entry)
                if verbose:
                    print(f"Expired entry: {entry.system_name} {entry.system_platform} {entry.system_version}")
        
