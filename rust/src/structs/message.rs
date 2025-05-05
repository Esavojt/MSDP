use uuid::Uuid;
#[derive(Debug, Clone)]
pub struct MessageV1 {
    pub version: u16,
    pub unique_id: Uuid,
    pub system_name: String,
    pub system_platform: String,
    pub system_version: String,
    pub keepalive_timer: u16,
    pub uptime: u32,
    pub load: [f32; 3],
}

impl MessageV1 {
    pub fn new(
        unique_id: Uuid,
        system_name: String,
        system_platform: String,
        system_version: String,
        keepalive_timer: u16,
        uptime: u32,
        load: [f32; 3],
    ) -> Self {
        MessageV1 {
            version: 1,
            unique_id,
            system_name,
            system_platform,
            system_version,
            keepalive_timer,
            uptime,
            load,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.unique_id.as_bytes());
        bytes.extend(&self.version.to_be_bytes());

        bytes.extend((self.system_name.len() as u16).to_be_bytes());
        bytes.extend(self.system_name.as_bytes());

        bytes.extend((self.system_platform.len() as u16).to_be_bytes());
        bytes.extend(self.system_platform.as_bytes());

        bytes.extend((self.system_version.len() as u16).to_be_bytes());
        bytes.extend(self.system_version.as_bytes());

        bytes.extend(&self.keepalive_timer.to_be_bytes());
        bytes.extend(&self.uptime.to_be_bytes());
        for &load in &self.load {
            bytes.extend(&((load * 100.0) as u16).to_be_bytes());
        }
        bytes
    }
    pub fn parse(bytes: &[u8]) -> Result<Self, u32> {
        let unique_id = Uuid::from_slice(&bytes[0..16]).map_err(|_| 1 as u32)?;
        let version = u16::from_be_bytes([bytes[16], bytes[17]]);

        let mut offset = 18;

        if version != 1 {
            return Err(2);
        }

        let system_name_len = u16::from_be_bytes([bytes[18], bytes[19]]) as u16;
        offset += 2;
        let system_name =
            String::from_utf8_lossy(&bytes[offset..offset + system_name_len as usize]).to_string();
        offset += system_name_len as usize;

        let system_platform_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as u16;
        offset += 2;
        let system_platform =
            String::from_utf8_lossy(&bytes[offset..offset + system_platform_len as usize])
                .to_string();
        offset += system_platform_len as usize;

        let system_version_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as u16;
        offset += 2;
        let system_version =
            String::from_utf8_lossy(&bytes[offset..offset + system_version_len as usize])
                .to_string();
        offset += system_version_len as usize;

        let keepalive_timer = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let uptime = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let mut load = [0.0; 3];
        for i in 0..3 {
            load[i] = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as f32 / 100.0;
            offset += 2;
        }

        Ok(MessageV1 {
            version,
            unique_id,
            system_name,
            system_platform,
            system_version,
            keepalive_timer,
            uptime,
            load,
        })
    }
}
