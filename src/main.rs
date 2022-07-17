use crate::SwitchSNMP::SwitchBase::CiscoSwitch::CiscoBaseSwitch;

mod SwitchSNMP;

fn main() {
  println!("SNMP Probe start...");
  
  let mut c = SwitchSNMP::Cat2960::Cat2960::new("192.168.178.204:161", "public");
  c.getPortState(10);
}
