# Récupération de la FIP publiction
data "openstack_networking_floatingip_v2" "fip_publi" {
  tags = ["FIP_Publi", "pf07-01", var.phase]
}

# Affichage de l'adresse IP flottante
output "fip_publi" {
  value = data.openstack_networking_floatingip_v2.fip_publi.address
}
