const MESSAGE_IDENTIFIER_MASK: u16 = 0b0000_0111_1111_1111;
pub type MessageIdentifier = u16;

bitfield! {
    pub struct TransmitMessageHeader([u8]);
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

impl TransmitMessageHeader<[u8; 2]> {
    /*pub fn new(identifier: impl CustomMessage) -> Self {
        let mut msg = TransmitMessageHeader([0; 2]);
        //msg.set_standard_identifier(identifier::IDENTIFIER & MESSAGE_IDENTIFIER_MASK);
        msg
    }*/
}

pub struct TransmitMessage<'d> {
    header: TransmitMessageHeader<[u8; 2]>,
    data: &'d [u8],
}
impl<'d> TransmitMessage<'d> {
    pub fn new(identifier: MessageIdentifier, data: &'d [u8]) -> Self {
        let mut header = TransmitMessageHeader([0; 2]);
        header.set_standard_identifier(identifier & MESSAGE_IDENTIFIER_MASK);
        TransmitMessage::<'d> {
            header: header,
            data: data,
        }
    }
}

bitfield! {
    pub struct ReceiveMessageHeader([u8]);
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

impl ReceiveMessageHeader<[u8; 3]> {
    /*pub fn new(identifier: impl CustomMessage) -> Self {
        let mut msg = ReceiveMessageHeader([0; 3]);
        //msg.set_standard_identifier((identifier as u16) & MESSAGE_IDENTIFIER_MASK);
        msg
    }*/
}
