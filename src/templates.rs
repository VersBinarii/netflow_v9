use std::fmt;

use super::formaters::{fmt_int, fmt_ipv4, fmt_ipv6, fmt_tcp_flags};

pub enum TemplateFieldType {
    InBytes(&'static str, fn(&[u8]) -> String),
    InPkts(&'static str, fn(&[u8]) -> String),
    Flows(&'static str, fn(&[u8]) -> String),
    Protocol(&'static str, fn(&[u8]) -> String),
    SrcTos(&'static str, fn(&[u8]) -> String),
    TCPFlags(&'static str, fn(&[u8]) -> String),
    L4SrcPort(&'static str, fn(&[u8]) -> String),
    IPv4SrcAddr(&'static str, fn(&[u8]) -> String),
    SrcMask(&'static str, fn(&[u8]) -> String),
    InputSNMP(&'static str, fn(&[u8]) -> String),
    L4DstPort(&'static str, fn(&[u8]) -> String),
    IPv4DstAddr(&'static str, fn(&[u8]) -> String),
    DstMask(&'static str, fn(&[u8]) -> String),
    OutputSNMP(&'static str, fn(&[u8]) -> String),
    IPv4NextHop(&'static str, fn(&[u8]) -> String),
    SrcAS(&'static str, fn(&[u8]) -> String),
    DstAS(&'static str, fn(&[u8]) -> String),
    BgpIPv4NextHop(&'static str, fn(&[u8]) -> String),
    MulDstPkts(&'static str, fn(&[u8]) -> String),
    MulDstBytes(&'static str, fn(&[u8]) -> String),
    LastSwitched(&'static str, fn(&[u8]) -> String),
    FirstSwitched(&'static str, fn(&[u8]) -> String),
    OutBytes(&'static str, fn(&[u8]) -> String),
    OutPkts(&'static str, fn(&[u8]) -> String),
    MinPktLength(&'static str, fn(&[u8]) -> String),
    MaxPktLength(&'static str, fn(&[u8]) -> String),
    IPv6SrcAddr(&'static str, fn(&[u8]) -> String),
    IPv6DstAddr(&'static str, fn(&[u8]) -> String),
    IPv6SrcMask(&'static str, fn(&[u8]) -> String),
    IPv6DstMask(&'static str, fn(&[u8]) -> String),
    IPv6FlowLabel(&'static str, fn(&[u8]) -> String),
    ICMPType(&'static str, fn(&[u8]) -> String),
    MulIGMPType(&'static str, fn(&[u8]) -> String),
    SamplingInterval(&'static str, fn(&[u8]) -> String),
    SamplingAlgorithm(&'static str, fn(&[u8]) -> String),
    FlowActiveTimeout(&'static str, fn(&[u8]) -> String),
    FlowInactiveTimeout(&'static str, fn(&[u8]) -> String),
    EngineType(&'static str, fn(&[u8]) -> String),
    EngineID(&'static str, fn(&[u8]) -> String),
    TotalBytesExp(&'static str, fn(&[u8]) -> String),
    TotalPktsExp(&'static str, fn(&[u8]) -> String),
    TotalFlowsExp(&'static str, fn(&[u8]) -> String),
    IPv4SrcPrefix(&'static str, fn(&[u8]) -> String),
    IPv4DstPrefix(&'static str, fn(&[u8]) -> String),
    MPLSTopLabelType(&'static str, fn(&[u8]) -> String),
    MPLSTopLabelIPAddr(&'static str, fn(&[u8]) -> String),
    FlowSamplerID(&'static str, fn(&[u8]) -> String),
    FlowSamplerMode(&'static str, fn(&[u8]) -> String),
    FlowSamplerRandomInterval(&'static str, fn(&[u8]) -> String),
    MinTTL(&'static str, fn(&[u8]) -> String),
    MaxTTL(&'static str, fn(&[u8]) -> String),
    IPv4Ident(&'static str, fn(&[u8]) -> String),
    InSrcMac(&'static str, fn(&[u8]) -> String),
    OutDstMac(&'static str, fn(&[u8]) -> String),
    SrcVLAN(&'static str, fn(&[u8]) -> String),
    DstVLAN(&'static str, fn(&[u8]) -> String),
    IPProtocolVersion(&'static str, fn(&[u8]) -> String),
    Direction(&'static str, fn(&[u8]) -> String),
    IPv6NextHop(&'static str, fn(&[u8]) -> String),
    BgpIPv6NextHop(&'static str, fn(&[u8]) -> String),
    IPv6OptionHeaders(&'static str, fn(&[u8]) -> String),
    MPLSLabel1(&'static str, fn(&[u8]) -> String),
    MPLSLabel2(&'static str, fn(&[u8]) -> String),
    MPLSLabel3(&'static str, fn(&[u8]) -> String),
    MPLSLabel4(&'static str, fn(&[u8]) -> String),
    MPLSLabel5(&'static str, fn(&[u8]) -> String),
    MPLSLabel6(&'static str, fn(&[u8]) -> String),
    MPLSLabel7(&'static str, fn(&[u8]) -> String),
    MPLSLabel8(&'static str, fn(&[u8]) -> String),
    MPLSLabel9(&'static str, fn(&[u8]) -> String),
    MPLSLabel10(&'static str, fn(&[u8]) -> String),
    InDstMAC(&'static str, fn(&[u8]) -> String),
    OutSrcMAC(&'static str, fn(&[u8]) -> String),
    IfName(&'static str, fn(&[u8]) -> String),
    IfDesc(&'static str, fn(&[u8]) -> String),
    ForwardingStatus(&'static str, fn(&[u8]) -> String),
    ReplicationFactor(&'static str, fn(&[u8]) -> String),
    Unimplemented(&'static str, fn(&[u8]) -> String),
}

impl fmt::Display for TemplateFieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enum_value = match *self {
            TemplateFieldType::InBytes(x, _) => x,
            TemplateFieldType::InPkts(x, _) => x,
            TemplateFieldType::Flows(x, _) => x,
            TemplateFieldType::Protocol(x, _) => x,
            TemplateFieldType::SrcTos(x, _) => x,
            TemplateFieldType::TCPFlags(x, _) => x,
            TemplateFieldType::L4SrcPort(x, _) => x,
            TemplateFieldType::IPv4SrcAddr(x, _) => x,
            TemplateFieldType::SrcMask(x, _) => x,
            TemplateFieldType::InputSNMP(x, _) => x,
            TemplateFieldType::L4DstPort(x, _) => x,
            TemplateFieldType::IPv4DstAddr(x, _) => x,
            TemplateFieldType::DstMask(x, _) => x,
            TemplateFieldType::OutputSNMP(x, _) => x,
            TemplateFieldType::IPv4NextHop(x, _) => x,
            TemplateFieldType::SrcAS(x, _) => x,
            TemplateFieldType::DstAS(x, _) => x,
            TemplateFieldType::BgpIPv4NextHop(x, _) => x,
            TemplateFieldType::MulDstPkts(x, _) => x,
            TemplateFieldType::MulDstBytes(x, _) => x,
            TemplateFieldType::LastSwitched(x, _) => x,
            TemplateFieldType::FirstSwitched(x, _) => x,
            TemplateFieldType::OutBytes(x, _) => x,
            TemplateFieldType::OutPkts(x, _) => x,
            TemplateFieldType::MinPktLength(x, _) => x,
            TemplateFieldType::MaxPktLength(x, _) => x,
            TemplateFieldType::IPv6SrcAddr(x, _) => x,
            TemplateFieldType::IPv6DstAddr(x, _) => x,
            TemplateFieldType::IPv6SrcMask(x, _) => x,
            TemplateFieldType::IPv6DstMask(x, _) => x,
            TemplateFieldType::IPv6FlowLabel(x, _) => x,
            TemplateFieldType::ICMPType(x, _) => x,
            TemplateFieldType::MulIGMPType(x, _) => x,
            TemplateFieldType::SamplingInterval(x, _) => x,
            TemplateFieldType::SamplingAlgorithm(x, _) => x,
            TemplateFieldType::FlowActiveTimeout(x, _) => x,
            TemplateFieldType::FlowInactiveTimeout(x, _) => x,
            TemplateFieldType::EngineType(x, _) => x,
            TemplateFieldType::EngineID(x, _) => x,
            TemplateFieldType::TotalBytesExp(x, _) => x,
            TemplateFieldType::TotalPktsExp(x, _) => x,
            TemplateFieldType::TotalFlowsExp(x, _) => x,

            TemplateFieldType::MinTTL(x, _) => x,
            TemplateFieldType::MaxTTL(x, _) => x,
            TemplateFieldType::IPv4Ident(x, _) => x,
            TemplateFieldType::InSrcMac(x, _) => x,
            TemplateFieldType::OutDstMac(x, _) => x,
            TemplateFieldType::SrcVLAN(x, _) => x,
            TemplateFieldType::DstVLAN(x, _) => x,
            TemplateFieldType::IPProtocolVersion(x, _) => x,
            TemplateFieldType::Direction(x, _) => x,
            TemplateFieldType::IPv6NextHop(x, _) => x,
            TemplateFieldType::BgpIPv6NextHop(x, _) => x,
            TemplateFieldType::IPv6OptionHeaders(x, _) => x,

            TemplateFieldType::InDstMAC(x, _) => x,
            TemplateFieldType::OutSrcMAC(x, _) => x,
            TemplateFieldType::IfName(x, _) => x,
            TemplateFieldType::IfDesc(x, _) => x,

            TemplateFieldType::ForwardingStatus(x, _) => x,
            TemplateFieldType::ReplicationFactor(x, _) => x,

            TemplateFieldType::Unimplemented(x, _) => x,

            _ => "Unknown value",
        };

        write!(f, "{}", enum_value)
    }
}

impl TemplateFieldType {
    pub fn get_parser(&self) -> fn(&[u8]) -> String {
        match *self {
            TemplateFieldType::InBytes(_, f) => f,
            TemplateFieldType::InPkts(_, f) => f,
            TemplateFieldType::Flows(_, f) => f,
            TemplateFieldType::Protocol(_, f) => f,
            TemplateFieldType::SrcTos(_, f) => f,
            TemplateFieldType::TCPFlags(_, f) => f,
            TemplateFieldType::L4SrcPort(_, f) => f,
            TemplateFieldType::IPv4SrcAddr(_, f) => f,
            TemplateFieldType::SrcMask(_, f) => f,
            TemplateFieldType::InputSNMP(_, f) => f,
            TemplateFieldType::L4DstPort(_, f) => f,
            TemplateFieldType::IPv4DstAddr(_, f) => f,
            TemplateFieldType::DstMask(_, f) => f,
            TemplateFieldType::OutputSNMP(_, f) => f,
            TemplateFieldType::IPv4NextHop(_, f) => f,
            TemplateFieldType::SrcAS(_, f) => f,
            TemplateFieldType::DstAS(_, f) => f,
            TemplateFieldType::BgpIPv4NextHop(_, f) => f,

            TemplateFieldType::LastSwitched(_, f) => f,
            TemplateFieldType::FirstSwitched(_, f) => f,

            TemplateFieldType::MinPktLength(_, f) => f,
            TemplateFieldType::MaxPktLength(_, f) => f,

            TemplateFieldType::IPv6SrcAddr(_, f) => f,
            TemplateFieldType::IPv6DstAddr(_, f) => f,
            TemplateFieldType::IPv6SrcMask(_, f) => f,
            TemplateFieldType::IPv6DstMask(_, f) => f,
            TemplateFieldType::IPv6FlowLabel(_, f) => f,
            TemplateFieldType::ICMPType(_, f) => f,

            TemplateFieldType::MinTTL(_, f) => f,
            TemplateFieldType::MaxTTL(_, f) => f,

            TemplateFieldType::IPProtocolVersion(_, f) => f,
            TemplateFieldType::Direction(_, f) => f,
            TemplateFieldType::IPv6NextHop(_, f) => f,
            TemplateFieldType::BgpIPv6NextHop(_, f) => f,
            TemplateFieldType::IPv6OptionHeaders(_, f) => f,

            TemplateFieldType::ForwardingStatus(_, f) => f,
            TemplateFieldType::ReplicationFactor(_, f) => f,
            TemplateFieldType::MulDstPkts(_, f) => f,
            TemplateFieldType::MulDstBytes(_, f) => f,
            TemplateFieldType::OutBytes(_, f) => f,
            TemplateFieldType::OutPkts(_, f) => f,
            TemplateFieldType::MulIGMPType(_, f) => f,
            TemplateFieldType::SamplingInterval(_, f) => f,
            TemplateFieldType::SamplingAlgorithm(_, f) => f,
            TemplateFieldType::FlowActiveTimeout(_, f) => f,
            TemplateFieldType::FlowInactiveTimeout(_, f) => f,
            TemplateFieldType::EngineType(_, f) => f,
            TemplateFieldType::EngineID(_, f) => f,
            TemplateFieldType::TotalBytesExp(_, f) => f,
            TemplateFieldType::TotalPktsExp(_, f) => f,
            TemplateFieldType::TotalFlowsExp(_, f) => f,

            TemplateFieldType::IPv4Ident(_, f) => f,
            TemplateFieldType::InSrcMac(_, f) => f,
            TemplateFieldType::OutDstMac(_, f) => f,
            TemplateFieldType::SrcVLAN(_, f) => f,
            TemplateFieldType::DstVLAN(_, f) => f,

            TemplateFieldType::InDstMAC(_, f) => f,
            TemplateFieldType::OutSrcMAC(_, f) => f,
            TemplateFieldType::IfName(_, f) => f,
            TemplateFieldType::IfDesc(_, f) => f,

            TemplateFieldType::Unimplemented(_, f) => f,

            _ => {
                println!("{}", *self);
                |_: &[u8]| "Unknown value".to_owned()
            }
        }
    }
}

impl From<u16> for TemplateFieldType {
    fn from(field: u16) -> TemplateFieldType {
        match field {
            1 => TemplateFieldType::InBytes("InBytes", fmt_int),
            2 => TemplateFieldType::InPkts("InPackets", fmt_int),
            3 => TemplateFieldType::Flows("Flows", fmt_int),
            4 => TemplateFieldType::Protocol("Protocol", fmt_int),
            5 => TemplateFieldType::SrcTos("Src Tos", fmt_int),
            6 => TemplateFieldType::TCPFlags("TCP Flags", fmt_tcp_flags),
            7 => TemplateFieldType::L4SrcPort("L4 Src port", fmt_int),
            8 => TemplateFieldType::IPv4SrcAddr("IPv4 Src Addr", fmt_ipv4),
            9 => TemplateFieldType::SrcMask("Src Mask", fmt_int),
            10 => TemplateFieldType::InputSNMP("Input SNMP", fmt_int),
            11 => TemplateFieldType::L4DstPort("L4 Dst port", fmt_int),
            12 => TemplateFieldType::IPv4DstAddr("IPv4 Dest Addr", fmt_ipv4),
            13 => TemplateFieldType::DstMask("Dest Mask", fmt_int),
            14 => TemplateFieldType::OutputSNMP("Output SNMP", fmt_int),
            15 => TemplateFieldType::IPv4NextHop("IPv4 Next Hop", fmt_ipv4),
            16 => TemplateFieldType::SrcAS("Src AS", fmt_int),
            17 => TemplateFieldType::DstAS("Dst AS", fmt_int),
            18 => {
                TemplateFieldType::BgpIPv4NextHop("BGP IPv4 Next Hop", fmt_ipv4)
            }
            19 => {
                TemplateFieldType::MulDstPkts("Multicast Dest Packets", fmt_int)
            }
            20 => {
                TemplateFieldType::MulDstBytes("Multicast Dest Bytes", fmt_int)
            }
            21 => TemplateFieldType::LastSwitched("Last Switched", fmt_int),
            22 => TemplateFieldType::FirstSwitched("First switched", fmt_int),
            23 => TemplateFieldType::OutBytes("Outgoing bytes", fmt_int),
            24 => TemplateFieldType::OutPkts("Outgoing packets", fmt_int),
            25 => TemplateFieldType::MinPktLength("Min packet length", fmt_int),
            26 => TemplateFieldType::MaxPktLength("Max packet length", fmt_int),
            27 => TemplateFieldType::IPv6SrcAddr("IPv6 Src Addr", fmt_ipv6),
            28 => TemplateFieldType::IPv6DstAddr("IPv6 Dst Addr", fmt_ipv6),
            29 => TemplateFieldType::IPv6SrcMask("IPv6 Src Mask", fmt_int),
            30 => TemplateFieldType::IPv6DstMask("IPv6 Dst Mask", fmt_int),
            31 => TemplateFieldType::IPv6FlowLabel("IPv6 Flow Label", fmt_int),
            32 => TemplateFieldType::ICMPType("ICMP type", fmt_int),
            33 => {
                TemplateFieldType::MulIGMPType("Multicast IGMP type", fmt_int)
            }
            34 => TemplateFieldType::SamplingInterval(
                "Sampling interval",
                fmt_int,
            ),
            35 => TemplateFieldType::SamplingAlgorithm(
                "Sampling algorithm",
                fmt_int,
            ),
            36 => TemplateFieldType::FlowActiveTimeout(
                "Flow active timeout",
                fmt_int,
            ),
            37 => TemplateFieldType::FlowInactiveTimeout(
                "Flow inactive timeout",
                fmt_int,
            ),
            38 => TemplateFieldType::EngineType("Engine type", fmt_int),
            39 => TemplateFieldType::EngineID("Engine Id", fmt_int),

            52 => TemplateFieldType::MinTTL("Min TTL", fmt_int),
            53 => TemplateFieldType::MaxTTL("Max TTL", fmt_int),

            60 => TemplateFieldType::IPProtocolVersion(
                "IP Proto version",
                fmt_int,
            ),
            61 => TemplateFieldType::Direction("Direction", fmt_int),
            62 => TemplateFieldType::IPv6NextHop("IPv6 Next Hop", fmt_ipv6),
            63 => {
                TemplateFieldType::BgpIPv6NextHop("BGP IPv6 Next Hop", fmt_ipv6)
            }
            64 => TemplateFieldType::IPv6OptionHeaders(
                "IPv6 Options header",
                fmt_int,
            ),

            89 => TemplateFieldType::ForwardingStatus(
                "Forwarding Status",
                fmt_int,
            ),

            99 => TemplateFieldType::ReplicationFactor(
                "Multicast replication factor",
                fmt_int,
            ),

            _ => {
                println!("Unknown fielt type: {}", field);
                TemplateFieldType::Unimplemented("Unknown Field Type", fmt_int)
            }
        }
    }
}
