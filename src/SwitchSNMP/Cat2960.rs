// use super::SwitchBase::{CiscoSwitch, Probe::{ProbeSNMP}, PortState};
use super::SwitchBase::CiscoSwitch::CiscoBaseSwitch;
use super::SwitchBase::Probe::ProbeSNMP::ProbeSNMP;
use super::SwitchBase::PortState;

const CAT2960_INTERFACE_STATUS: [u32; 10] = [1,3,6,1,2,1,2,2,1,8,];

pub struct Cat2960 { _probe: ProbeSNMP }

impl CiscoBaseSwitch for Cat2960 {
  fn new(addr:&str, community:&str) -> Self {
    Cat2960 { 
      _probe: ProbeSNMP::new(addr, community) 
    }
  }

  fn getPortState(&mut self, port:u32) -> PortState::State {
    let portId: [u32; 11] = {
      let mut portID: [u32; 11] = [0; 11];
      let (first, second) = portID.split_at_mut(CAT2960_INTERFACE_STATUS.len());
      first.copy_from_slice(&CAT2960_INTERFACE_STATUS);
      second.copy_from_slice(&[(10000+port)]);
      portID
    };

    println!("Port ID: {:?}", portId);
    return PortState::State::UNKNOWN;
  }

  fn getPortStateVlan(&mut self, vlan:u32) -> PortState::State {
    return PortState::State::UNKNOWN;
  }
}