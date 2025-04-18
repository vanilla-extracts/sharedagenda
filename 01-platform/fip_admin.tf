# Récupération de la FIP administration
data "openstack_networking_floatingip_v2" "fip_admin" {
  tags = ["FIP_Admin", "pf07-01", var.phase]
}

# Affichage de l'adresse IP flottante
output "fip_admin" {
#  value = local.fip_administration_network_ip
  value = data.openstack_networking_floatingip_v2.fip_admin.address
}
