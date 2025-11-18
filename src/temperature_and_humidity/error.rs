use embassy_dht_sensor::DHTSensorError;

#[derive(Clone, Debug)]
pub enum FormattableDHTSensorError {
    DHTSensorError(DHTSensorError),
}

impl defmt::Format for FormattableDHTSensorError {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        match self {
            Self::DHTSensorError(DHTSensorError::NoData) => defmt::write!(fmt, "{}", "NoData"),
            Self::DHTSensorError(DHTSensorError::ChecksumError) => {
                defmt::write!(fmt, "{}", "ChecksumError")
            }
            Self::DHTSensorError(DHTSensorError::InvalidData) => {
                defmt::write!(fmt, "{}", "InvalidData")
            }
            Self::DHTSensorError(DHTSensorError::Timeout) => defmt::write!(fmt, "{}", "Timeout"),
        }
    }
}

impl From<DHTSensorError> for FormattableDHTSensorError {
    fn from(err: DHTSensorError) -> Self {
        FormattableDHTSensorError::DHTSensorError(err)
    }
}
