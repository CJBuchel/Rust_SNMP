use std::time::Duration;
use snmp::{SyncSession, SnmpPdu};


pub struct ProbeSNMP {
  addr:  String,
  community:  String,
  session: SyncSession,
}

/**
 * Basic SNMP Read only probe commands
 */
impl ProbeSNMP {
  pub const TIMEOUT: Duration = Duration::from_secs(2);

  /**
   * New probe
   */
  pub fn new(addr:&str, community:&str) -> Self {
    ProbeSNMP {
      addr: (addr.to_string()),
      community: (community.to_string()),
      session: SyncSession::new(addr, community.as_bytes(), Some(ProbeSNMP::TIMEOUT), 0).unwrap()
    }
  }

  /**
   * Get from raw OID value
   */
  pub fn get(&mut self, oid: &[u32]) -> SnmpPdu {
    let mut response = self.session.get(oid).unwrap();
    return response;
  }

  /**
   * Get  Next from OID value
   */
  pub fn getNext(&mut self, oid: &[u32]) -> SnmpPdu {
    let mut response = self.session.getnext(oid).unwrap();
    return response;
  }

  /**
   * Get Bulk data from OID base or multiple OID's
   */
  pub fn getBulk(&mut self, oids: &[&[u32]], non_repeaters: u32, max_repetitions: u32) -> SnmpPdu {
    let mut response = self.session.getbulk(oids, non_repeaters, max_repetitions).unwrap();
    return response;
  }
}