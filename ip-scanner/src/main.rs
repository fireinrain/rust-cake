// use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
// use pnet::packet::ipv4::MutableIpv4Packet;
// use pnet::packet::ethernet::MutableEthernetPacket;
// use pnet::transport::{transport_channel, TransportReceiver, TransportSender};
// use pnet::util::checksum;
//
// use std::net::{IpAddr, Ipv4Addr};
// use std::time::Duration;
//
// fn main() {
//     let destination_ip = "39.156.66.10";
//     let destination_port = 443;
//
//     let (mut sender, receiver) = transport_channel(4096, Default::default())
//         .expect("Failed to create channel for sending/receiving packets");
//
//     let syn_packet = build_syn_packet(
//         destination_ip.parse().expect("Invalid destination IP address"),
//         destination_port,
//     );
//
//     // Send the SYN packet
//     sender
//         .send_to(syn_packet, IpAddr::V4(destination_ip.parse().unwrap()))
//         .expect("Failed to send packet");
//
//     // Set a timeout for receiving response packets
//     receiver
//         .set_timeout(Some(Duration::from_secs(2)))
//         .expect("Failed to set timeout");
//
//     let mut buffer = [0u8; 4096];
//     while let Ok((size, _)) = receiver.recv_from(&mut buffer) {
//         // Process received packets, checking for SYN-ACK response
//         process_received_packet(&buffer[..size], destination_ip, destination_port);
//
//         // Exit loop if no more packets are received
//         if size == 0 {
//             break;
//         }
//     }
// }
//
// fn build_syn_packet(destination_ip: IpAddr, destination_port: u16) -> Vec<u8> {
//     // Create Ethernet packet
//     let mut ethernet_buffer = [0u8; MutableEthernetPacket::minimum_packet_size()];
//     let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();
//
//     // Set source and destination MAC addresses (arbitrary values)
//     ethernet_packet.set_source([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
//     ethernet_packet.set_destination([0x00, 0x11, 0x22, 0x33, 0x44, 0x66]);
//
//     // Create IPv4 packet
//     let mut ipv4_buffer = [0u8; MutableIpv4Packet::minimum_packet_size()];
//     let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_buffer).unwrap();
//
//     // Set source and destination IP addresses
//     ipv4_packet.set_source(Ipv4Addr::new(127, 0, 0, 1));
//     ipv4_packet.set_destination(match destination_ip {
//         IpAddr::V4(ipv4) => ipv4,
//         _ => panic!("IPv6 addresses are not supported"),
//     });
//
//     // Set IPv4 packet properties
//     ipv4_packet.set_version(4);
//     ipv4_packet.set_header_length(5);
//     ipv4_packet.set_total_length((ipv4_packet.packet_size() + MutableTcpPacket::minimum_packet_size()) as u16);
//     ipv4_packet.set_ttl(64);
//     ipv4_packet.set_protocol(6); // 6 for TCP
//
//     // Create TCP packet
//     let mut tcp_buffer = [0u8; MutableTcpPacket::minimum_packet_size()];
//     let mut tcp_packet = MutableTcpPacket::new(&mut tcp_buffer).unwrap();
//
//     // Set source and destination ports
//     tcp_packet.set_source(12345);
//     tcp_packet.set_destination(destination_port);
//
//     // Set TCP packet properties
//     tcp_packet.set_data_offset(8); // 8 bytes for TCP header
//     tcp_packet.set_flags(TcpFlags::SYN);
//     tcp_packet.set_window(64240);
//
//     // Calculate checksums
//     let ipv4_checksum = checksum(&ipv4_packet.to_immutable());
//     ipv4_packet.set_checksum(ipv4_checksum);
//
//     let tcp_checksum = checksum(&tcp_packet.to_immutable(),
//                                 &ipv4_packet.get_source(),
//                                 &ipv4_packet.get_destination(),
//                                 &[]); // No payload
//     tcp_packet.set_checksum(tcp_checksum);
//
//     // Build final packet by concatenating Ethernet, IPv4, and TCP packets
//     let mut packet = Vec::with_capacity(ethernet_packet.packet().len()
//         + ipv4_packet.packet().len()
//         + tcp_packet.packet().len());
//     packet.extend_from_slice(ethernet_packet.packet());
//     packet.extend_from_slice(ipv4_packet.packet());
//     packet.extend_from_slice(tcp_packet.packet());
//
//     packet
// }
//
// fn process_received_packet(packet: &[u8], destination_ip: &str, destination_port: u16) {
//     // Parse received packet into Ethernet, IPv4, and TCP packets
//     let ethernet_packet = pnet::packet::ethernet::EthernetPacket::new(packet).unwrap();
//     let ipv4_packet = pnet::packet::ipv4::Ipv4Packet::new(ethernet_packet.payload()).unwrap();
//     let tcp_packet = pnet::packet::tcp::TcpPacket::new(ipv4_packet.payload()).unwrap();
//
//     // Check if the received packet is a SYN-ACK response
//     if tcp_packet.get_destination() == 12345
//         && tcp_packet.get_source() == destination_port
//         && tcp_packet.get_flags() == TcpFlags::SYN | TcpFlags::ACK
//     {
//         println!(
//             "Received SYN-ACK response from {}:{}. The port is open.",
//             destination_ip, destination_port
//         );
//     }
// }
