output "instance_id" {
  description = "Identifiant de l'instance "
  value = openstack_compute_instance_v2.instance.*.id
}

output "instance_metadata" {
  description = "Metadata de l'instance "
  value = openstack_compute_instance_v2.instance.*.metadata
}

output "instance_hostname" {
  description = "Nom de l'instance"
  value       = openstack_compute_instance_v2.instance.*.name
}

output "instance_extra_disks" {
  description = "Liste des disques attachés"
  value = local.attachments
}

output "instance_admin_ip" {
  description = "Ip d'admin de l'instance" 
  value       = ((var.is_admin_network != false) && (var.server_count > 0)) ? flatten(openstack_networking_port_v2.admin_network_port.*.all_fixed_ips) : null
}

output "instance_pub_ip" {
  description = "Ip de publication de l'instance"
  value       = ((var.is_pub_network != false) && (var.server_count > 0)) ? flatten(openstack_networking_port_v2.pub_network_port.*.all_fixed_ips) : null
}

output instance_data_ip {
  description = "Ip de donnée de l'instance"
  value       = ((var.is_data_network != false) && (var.server_count > 0)) ? flatten(openstack_networking_port_v2.data_network_port.*.all_fixed_ips) : null
}
