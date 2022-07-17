use super::Probe::{ProbeSNMP};
use super::PortState;
use snmp::{Value};

// 
// Most cisco switches use similar OID/MIB tree.
// But not everything crosses over
// So instead lets build some traits
// 

const CISCO_SYSTEM_OID: [u32; 7] = [1,3,6,1,2,1,1,]; // base of oid tree (rise ye tarnished)
const CISCO_INTERFACES_OID: [u32; 7] = [1,3,6,1,2,1,2,]; // n ports
// const CISCO_CONNECTIONS_OID: [u32; 15] = [1,3,6,1,2,1,4,1,9,9,147,1,2,2,2,]; // n connections
const CISCO_DESCRIPTION_OID: [u32; 8] = [1,3,6,1,2,1,1,1,]; // description of device

pub struct CiscoBaseSwitch { _probe: ProbeSNMP::ProbeSNMP }
impl CiscoBaseSwitch {
  /**
   * Create new switch
   */
  pub fn new(addr:&str, community:&str) -> Self {
    CiscoBaseSwitch {
      _probe: ProbeSNMP::ProbeSNMP::new(addr, community) 
    }
  }

  /**
   * Get Description
   */
  pub fn getDescription(&mut self) -> String {
    let mut response = self._probe.getNext(&CISCO_DESCRIPTION_OID);
    if let Some((_oid, Value::OctetString(_val))) = response.varbinds.next() {
      return String::from_utf8_lossy(_val).to_string();
    }

    return "".to_string();
  }

  /**
   * Get number of ports on switch
   * @returns number of physical ports on switch (including uplink) or 0 if fail
   */
  pub fn getInterfaces(&mut self) -> u32 {
    let mut response = self._probe.getNext(&CISCO_INTERFACES_OID);
    if let Some((_oid, Value::Integer(_val))) = response.varbinds.next() {
      return _val as u32;
    }

    return 0;
  }

  // /**
  //  * Get number of connections to switch
  //  * returns number of connections or returns 0 if fail/no connections
  //  */
  // pub fn getConnections(&mut self) -> u32 {
  //   let mut response = self._probe.getNext(&CISCO_CONNECTIONS_OID);
  //   if let Some((_oid, _val)) = response.varbinds.next() {
  //     match _val {
  //       Value::Integer(n) => return n as u32,
  //       _ => return 0,
  //     }
  //   }

  //   return 0;
  // }
}