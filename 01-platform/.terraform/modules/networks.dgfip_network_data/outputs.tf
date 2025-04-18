output "network_id" {

  description = "Id of the network created"

  value = join(" ", openstack_networking_network_v2.created.*.id)
}


output "router_ip" {
  description = "IP du router"
  value = (var.is_external_network == true ) ? openstack_networking_router_v2.created.0.external_fixed_ip.*.ip_address : null
}
