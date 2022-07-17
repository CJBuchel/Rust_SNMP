use std::{vec, f32::INFINITY};
use std::time::Duration;
use snmp::{SyncSession, Value, SnmpPdu, Varbinds};

// const switch_descr_oid: &[u32; 8] = &[1,3,6,1,2,1,1,1,];
// const switch_addr: &str = "192.168.178.204:161";
// const community: &[u8; 6] = b"public";

struct SwitchProbe {
  addr: String,
  community: String,
  session: SyncSession
}

impl SwitchProbe {
  const SYSTEM_OID: [u32; 7] = [1,3,6,1,2,1,1,];
  const INTERFACES_OID: [u32; 7] = [1,3,6,1,2,1,2,];
  const CONNECTIONS_OID: [u32; 8] = [1,3,6,1,2,1,4,1,];
  const DESCRIPTION_OID: [u32; 8] = [1,3,6,1,2,1,1,1,];
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

}

fn main() {
  println!("Probe start...");

  let mut probe = SwitchProbe::new("192.168.178.204:161", "public");

  let mut response = probe.getNext(&SwitchProbe::INTERFACES_OID);

  if let Some((_oid, val)) = response.varbinds.next() {
    println!("Number of ports: {:?}", val);
  }

  let mut response = probe.getNext(&SwitchProbe::CONNECTIONS_OID);
  if let Some((_oid, val)) = response.varbinds.next() {
    println!("Number of connections: {:?}", val);
  }

  // for (name, val) in response.varbinds.next() {
  //   println!("{} => {:?}", name, val);
  // }

  // for (name, val) in response.varbinds. {
  //   println!("{} => {:?}", name, val);
  // }
  // probe_switch(probe);
}