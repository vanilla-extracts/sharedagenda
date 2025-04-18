resource "openstack_networking_secgroup_v2" "sg" {
  name                 = format("%s-sg-%s-%s", var.pf_prefixe, var.sg_objet, var.phase)
  description          = var.sg_description
  delete_default_rules = var.delete_default_rules
}



resource "openstack_networking_secgroup_rule_v2" "rule" {
  count             = length(var.sg_rules)
  port_range_min    = lookup(var.sg_rules[count.index], "port_range_min")
  port_range_max    = lookup(var.sg_rules[count.index], "port_range_max")
  protocol          = lookup(var.sg_rules[count.index], "protocol")
  direction         = lookup(var.sg_rules[count.index], "direction")
  remote_ip_prefix  = lookup(var.sg_rules[count.index], "remote_group") == "" ? lookup(var.sg_rules[count.index], "remote_ip_prefix") : ""
  remote_group_id   = lookup(var.sg_rules[count.index], "remote_group")
  ethertype         = "IPv4"
  security_group_id = openstack_networking_secgroup_v2.sg.id
 }