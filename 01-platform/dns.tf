# Récupère les informations de la zone
# Ne fonctionne que s'il n'y qu'une seule zone
data "openstack_dns_zone_v2" "zone_dns" {
}

# Affiche les informations sur la zone
output "DNS_zone" {
    value = "${data.openstack_dns_zone_v2.zone_dns.name}"
}

# Ajoute un recordset pour la FIP d'administration
resource "openstack_dns_recordset_v2" "dns_admin" {
  zone_id     = data.openstack_dns_zone_v2.zone_dns.id
  name        = "admin-${var.pf_prefixe}.${data.openstack_dns_zone_v2.zone_dns.name}"
  description = "Accès administration"
  ttl         = 3600
  type        = "A"
  records     = [ data.openstack_networking_floatingip_v2.fip_admin.address ]
}

# Affiche le nom d'administration
output "DNS_administration" {
    value = "${openstack_dns_recordset_v2.dns_admin.name}"
}

# Ajoute un recordset pour la FIP de publication
resource "openstack_dns_recordset_v2" "dns_publi" {
  zone_id     = data.openstack_dns_zone_v2.zone_dns.id
  name        = "www-${var.pf_prefixe}.${data.openstack_dns_zone_v2.zone_dns.name}"
  description = "Accès publication"
  ttl         = 3600
  type        = "A"
  records     = [ data.openstack_networking_floatingip_v2.fip_publi.address ]
}

# Affiche le nom de publication
output "DNS_publication" {
    value = "${openstack_dns_recordset_v2.dns_publi.name}"
}

# Ajoute un recordset pour Bastion
resource "openstack_dns_recordset_v2" "dns_bastion" {
  zone_id     = data.openstack_dns_zone_v2.zone_dns.id
  name        = "${module.vm-bastion.instance_hostname[0]}.${data.openstack_dns_zone_v2.zone_dns.name}"
  description = "IP internes - Bastion"
  ttl         = 3600
  type        = "A"
  records     = [ module.vm-bastion.instance_admin_ip[0] ]
}
