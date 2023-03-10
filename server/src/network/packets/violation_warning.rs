use bytes::Bytes;
use bytes::BytesMut;

use crate::network::packets::ConnectedPacket;
use common::bail;
use common::Deserialize;
use common::ReadExtensions;
use common::{VError, VResult};

#[derive(Debug, Copy, Clone)]
pub enum ViolationType {
    Malformed,
}

impl TryFrom<i32> for ViolationType {
    type Error = VError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Malformed,
            _ => bail!(BadPacket, "Invalid violation type {}", value),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ViolationSeverity {
    Warning,
    FinalWarning,
    TerminatingConnection,
}

impl TryFrom<i32> for ViolationSeverity {
    type Error = VError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Warning,
            1 => Self::FinalWarning,
            2 => Self::TerminatingConnection,
            _ => bail!(BadPacket, "Invalid violation severity {}", value),
        })
    }
}

#[derive(Debug)]
pub struct ViolationWarning {
    /// Type of the violation.
    pub warning_type: ViolationType,
    /// Severity of the violation.
    pub severity: ViolationSeverity,
    /// ID of the invalid packet.
    pub packet_id: i32,
    /// Description of the violation.
    pub context: String,
}

impl ConnectedPacket for ViolationWarning {
    const ID: u32 = 0x9c;
}

impl Deserialize for ViolationWarning {
    fn deserialize(mut buffer: Bytes) -> VResult<Self> {
        tracing::debug!("{:x?}", buffer.as_ref());

        let warning_type = ViolationType::try_from(buffer.get_var_i32()?)?;
        let severity = ViolationSeverity::try_from(buffer.get_var_i32()?)?;
        let packet_id = buffer.get_var_i32()?;
        let context = buffer.get_string()?;

        Ok(Self {
            warning_type,
            severity,
            packet_id,
            context,
        })
    }
}
