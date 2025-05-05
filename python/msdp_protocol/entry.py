class Entry:

    def __init__(self, unique_id=None, system_name=None, system_platform=None, system_version=None, keepalive_timer=10, address=None, last_seen=None, uptime=0, load=[0, 0, 0]):
        self.unique_id = unique_id
        self.system_name = system_name
        self.system_platform = system_platform
        self.system_version = system_version
        self.keepalive_timer = keepalive_timer
        self.address = address
        self.last_seen = last_seen
        self.uptime = uptime
        self.load = load
        self.update_expiration_time()

    def __repr__(self):
        return f"Entry(unique_id={self.unique_id}, system_name={self.system_name}, system_platform={self.system_platform}, system_version={self.system_version}, keepalive_timer={self.keepalive_timer}, address={self.address}, last_seen={self.last_seen}, uptime={self.uptime})"
    
    def update_expiration_time(self):
        self.expiration_time = self.last_seen + self.keepalive_timer * 2

    def to_dict(self):
        return {
            "unique_id": str(self.unique_id),
            "system_name": self.system_name,
            "system_platform": self.system_platform,
            "system_version": self.system_version,
            "keepalive_timer": self.keepalive_timer,
            "address": self.address,
            "last_seen": self.last_seen,
            "uptime": self.uptime,
            "load": self.load
        }