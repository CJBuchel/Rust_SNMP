use std::{vec, f32::INFINITY};
use std::time::Duration;
use snmp::{SyncSession, Value, SnmpPdu, Varbinds};

const switch_descr_oid: &[u32; 8] = &[1,3,6,1,2,1,1,1,];
const switch_addr: &str = "192.168.178.204:161";
const community: &[u8; 6] = b"public";

struct SwitchProbe {
  addr: String,
  community: String,
  session: SyncSession
}

impl SwitchProbe {
  const SYSTEM_OID: [u32; 7] = [1,3,6,1,2,1,1,]; // base of oid tree (rise ye tarnished)
  const INTERFACES_OID: [u32; 7] = [1,3,6,1,2,1,2,]; // n connections
  const CONNECTIONS_OID: [u32; 8] = [1,3,6,1,2,1,4,1,]; // forgot what this does
  const DESCRIPTION_OID: [u32; 8] = [1,3,6,1,2,1,1,1,]; // description of device

  const CISCO_VLAN_MEMBERSHIP_OID: [u32; 14] = [1,3,6,1,4,1,9,9,68,1,2,2,1,2,];
  const CISCO_PORT_STATUS_OID: [u32; 10] = [1,3,6,1,2,1,2,2,1,8,];
  
  const TIMEOUT: Duration = Duration::from_secs(2);
  /**
   * Get new probe
   */
  fn new(addr:&str, community:&str) -> Self {
    SwitchProbe { 
      addr: (addr.to_string()), 
      community: (community.to_string()), 
      session: SyncSession::new(addr, community.as_bytes(), Some(SwitchProbe::TIMEOUT), 0).unwrap()
    }
  }

  /**
   * Get from oid value
   */
  fn get(&mut self, oid: &[u32]) -> SnmpPdu {
    let mut response = self.session.get(oid).unwrap();
    return response;
  }

  /**
   * Get Next from oid value
   */
  fn getNext(&mut self, oid: &[u32]) -> SnmpPdu {
    let mut response = self.session.getnext(oid).unwrap();
    return response;
  }

  /**
   * Get SNMP bulk
   */
  fn getBulk(&mut self, oids: &[&[u32]], non_repeaters: u32, max_repetitions: u32) -> SnmpPdu {
    let mut response = self.session.getbulk(oids, non_repeaters, max_repetitions).unwrap();
    return response;
  }

  /**
   * Get the status of port from vlan number
   * @returns bool
   */
  fn getPortState(&mut self, vlan:u32) -> PortState::State {
    let mut portNum:u32 = 0;

    // Get number of ports
    let mut response = self.getNext(&SwitchProbe::INTERFACES_OID);
    if let Some((_oid, Value::Integer(n_ports))) = response.varbinds.next() {
      portNum = n_ports as u32;
    }
    println!("Number of ports: {}", portNum);

    // Get all vlans on each port
    response = self.getBulk(&[&SwitchProbe::CISCO_VLAN_MEMBERSHIP_OID], 0, portNum);
    for (_oid, _vlan) in response.varbinds {
      match _vlan {
        // Value::Integer(n) => println!("{} => VLAN: {}", _oid, n),
        _ => println!("")
      }
    }

    // Get state of ports
    response = self.getBulk(&[&SwitchProbe::CISCO_PORT_STATUS_OID], 0, portNum);
    for (_oid, _state) in response.varbinds {
      println!("{} => {:?}", _oid, _state);
    }

    return false;
  }

}

fn main() {
  println!("Probe start...");

  let mut probe = SwitchProbe::new("192.168.178.204:161", "public");


  let mut response = probe.getBulk(&[&SwitchProbe::CISCO_VLAN_MEMBERSHIP_MIB], 0, 100);
  if let Some((_oid, val)) = response.varbinds.next() {
    println!("VLAN Test: {:?}", val);
  }

  probe.getPortStatusVLAN(10);

  for (name, val) in response.varbinds. {
    println!("{} => {:?}", name, val);
  }
  probe_switch(probe);
}
