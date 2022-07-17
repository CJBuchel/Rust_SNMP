use super::Probe::{ProbeSNMP, self};
use super::PortState;
use snmp::{Value};

// 
// Most cisco switches use similar OID/MIB tree.
// But not everything crosses over
// So instead lets build some traits
// 

const CISCO_SYSTEM_OID: [u32; 7] = [1,3,6,1,2,1,1,]; // base of oid tree (rise ye tarnished)
const CISCO_INTERFACES_OID: [u32; 7] = [1,3,6,1,2,1,2,]; // n ports
const CISCO_CONNECTIONS_OID: [u32; 15] = [1,3,6,1,2,1,4,1,9,9,147,1,2,2,2,]; // n connections
const CISCO_DESCRIPTION_OID: [u32; 8] = [1,3,6,1,2,1,1,1,]; // description of device

pub trait CiscoBaseSwitch {
  /**
   * Create new switch
   */
  fn new(addr:&str, community:&str) -> Self;

  /**
   * Get Description
   */
  fn getDescription(&mut self, mut probe:ProbeSNMP::ProbeSNMP) -> String {
    let mut response = probe.getNext(&CISCO_INTERFACES_OID);
    if let Some((_oid, Value::OctetString(_val))) = response.varbinds.next() {
      return String::from_utf8_lossy(_val).to_string();
    }

    return "".to_string();
  }

  /**
   * Get number of ports on switch
   * @returns number of physical ports on switch (including uplink) or 0 if fail
   */
  fn getInterfaces(&mut self, mut probe:ProbeSNMP::ProbeSNMP) -> u32 {
    let mut response = probe.getNext(&CISCO_INTERFACES_OID);
    if let Some((_oid, Value::Integer(_val))) = response.varbinds.next() {
      return _val as u32;
    }

    return 0;
  }

  /**
   * Get number of connections to switch
   * returns number of connections or returns 0 if fail/no connections
   */
  fn getConnections(&mut self, mut probe:ProbeSNMP::ProbeSNMP) -> u32 {
    let mut response = probe.getNext(&CISCO_CONNECTIONS_OID);
    if let Some((_oid, _val)) = response.varbinds.next() {
      match _val {
        Value::Integer(n) => return n as u32,
        _ => return 0,
      }
    }

    return 0;
  }

  //
  // Generic traits for cisco switch
  //
  fn getPortState(&mut self, port:u32) -> PortState::State;
  fn getPortStateVlan(&mut self, vlan:u32) -> PortState::State;
}