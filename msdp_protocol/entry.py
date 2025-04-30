class Entry:

    def __init__(self, unique_id=None, system_name=None, system_platform=None, system_version=None, keepalive_timer=10, address=None, last_seen=None):
        self.unique_id = unique_id
        self.system_name = system_name
        self.system_platform = system_platform
        self.system_version = system_version
        self.keepalive_timer = keepalive_timer
        self.address = address
        self.last_seen = last_seen
        self.update_expiration_time()

    def __repr__(self):
        return f"Entry(name={self.name}, description={self.description})"
    
    def update_expiration_time(self):
        self.expiration_time = self.last_seen + self.keepalive_timer * 2