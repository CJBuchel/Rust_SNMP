// use super::SwitchBase::{CiscoSwitch, Probe::{ProbeSNMP}, PortState};
use super::SwitchBase::CiscoSwitch::CiscoBaseSwitch;
use super::SwitchBase::Probe::ProbeSNMP::ProbeSNMP;
use super::SwitchBase::PortState;
use snmp::{Value, ObjectIdentifier};

const CAT2960_INTERFACE_STATUS: [u32; 10] = [1,3,6,1,2,1,2,2,1,8,];
const CAT2960_VLAN_MEMBERSHIP: [u32; 14] = [1,3,6,1,4,1,9,9,68,1,2,2,1,2,];

pub struct Cat2960 { _probe: ProbeSNMP, pub base: CiscoBaseSwitch }

impl Cat2960 {
  pub fn new(addr:&str, community:&str) -> Self {
    Cat2960 { 
      _probe: ProbeSNMP::new(addr, community),
      base: CiscoBaseSwitch::new(addr, community)
    }
  }

  pub fn getPortStateID(&mut self, oid:&[u32]) -> PortState::State {
    let mut response = self._probe.get(oid);
    if let Some((_oid, Value::Integer(_state))) = response.varbinds.next() {
      return PortState::getState(_state as i32);
    }
    return PortState::State::UNKNOWN;
  }

  pub fn getPortState(&mut self, port:u32) -> PortState::State {
    let portId: [u32; 11] = {
      let mut portID: [u32; 11] = [0; 11];
      let (first, second) = portID.split_at_mut(CAT2960_INTERFACE_STATUS.len());
      first.copy_from_slice(&CAT2960_INTERFACE_STATUS);
      second.copy_from_slice(&[(10000+port)]);
      portID
    };

    return self.getPortStateID(&portId);
  }

  pub fn getPortStateVlan(&mut self, vlan:u32) -> PortState::State {
    let numPorts:u32 = self.base.getInterfaces();
    let mut response = self._probe.getBulk(&[&CAT2960_VLAN_MEMBERSHIP], 0, numPorts);
    // Match to the vlan
    for (_oid, _vlan) in response.varbinds {
      match _vlan {
        Value::Integer(n) => {
          if n as u32 == vlan {
            let split = _oid.to_string().clone();
            let split = split.split(".");
            let portNum:u32 = split.last().unwrap().parse::<u32>().unwrap();
            return self.getPortState(portNum-10000);
          }
        },
        _ => ()
      }
    }
    return PortState::State::UNKNOWN;
  }

}