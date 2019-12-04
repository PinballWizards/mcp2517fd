type WordSize = u8;

pub const TX_HEADER_SIZE: usize = 8;
pub const RX_HEADER_SIZE: usize = 12;
pub const MAX_BUFFER_SIZE: usize = 48;

const MESSAGE_IDENTIFIER_MASK: u16 = 0b0000_0111_1111_1111;
pub type MessageIdentifier = u16;

bitfield! {
    pub struct TxHeader([WordSize]);
    impl Debug;
    u8;
    // T0
    u16, standard_identifier, set_standard_identifier: 10, 0;
    u32, extended_identifier, set_extended_identifier: 28, 10;
    sid11, _: 29, 29;
    // T1
    pub data_length_code, set_data_length_code: 35, 32;
    pub identifier_extension, _: 36;
    pub remote_transmission_request, _: 37;
    pub bit_rate_switched, _: 38;
    pub fd_frame, _: 39;
    pub error_status_indicator, _: 40;
    pub sequence, set_sequence: 47, 41;
}

pub struct TransmitMessage {
    header: TxHeader<[WordSize; TX_HEADER_SIZE]>,
    data: [WordSize; MAX_BUFFER_SIZE],
}
impl TransmitMessage {
    pub fn new(identifier: MessageIdentifier, data: [WordSize; MAX_BUFFER_SIZE]) -> Self {
        let mut header = TxHeader([0; TX_HEADER_SIZE]);
        header.set_standard_identifier(identifier & MESSAGE_IDENTIFIER_MASK);
        TransmitMessage {
            header: header,
            data: data,
        }
    }

    pub fn bytes(self) -> (usize, [WordSize; TX_HEADER_SIZE + MAX_BUFFER_SIZE]) {
        (0, [0u8; TX_HEADER_SIZE + MAX_BUFFER_SIZE])
    }
}

bitfield! {
    pub struct RxHeader([WordSize]);
    impl Debug;
    u8;
    // T0
    pub u16, standard_identifier, set_standard_identifier: 10, 0;
    pub u32, extended_identifier, set_extended_identifier: 28, 10;
    sid11, _: 29, 29;
    // T1
    pub data_length_code, set_data_length_code: 35, 32;
    pub identifier_extension, _: 36;
    pub remote_transmission_request, _: 37;
    pub bit_rate_switched, _: 38;
    pub fd_frame, _: 39;
    pub error_status_indicator, _: 40;
    pub filter_hit, _: 47, 43;
    // T2
    pub u32, timestamp, _: 95, 64;
}

pub struct ReceiveMessage {
    header: RxHeader<[WordSize; RX_HEADER_SIZE]>,
    data: [WordSize; MAX_BUFFER_SIZE],
}
impl ReceiveMessage {}
