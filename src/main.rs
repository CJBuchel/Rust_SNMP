mod SwitchSNMP;

fn main() {
  println!("SNMP Cat2960 Probe start...");
  
  let mut c = SwitchSNMP::Cat2960::Cat2960::new("192.168.178.204:161", "public");

  println!("{}\n", c.base.getDescription());
  println!("N Ports: {}", c.base.getInterfaces());
  println!("State of port 15: {:?}", c.getPortState(15)); // specific port

  println!("Port State on Vlan 10: {:?}", c.getPortStateVlan(10)); // port on vlan (gets the first port detected on vlan)
  println!("Port State on Vlan 10: {:?}", c.getPortStateVlan(20));
  println!("Port State on Vlan 10: {:?}", c.getPortStateVlan(30));
  println!("Port State on Vlan 10: {:?}", c.getPortStateVlan(40));
  println!("Port State on Vlan 10: {:?}", c.getPortStateVlan(50));
  println!("Port State on Vlan 10: {:?}", c.getPortStateVlan(60));
}
