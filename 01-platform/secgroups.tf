# Definition des security groups

# Le module terraform-openstack-bastion crée un security group pour bastion

module "sg-admin" {
  source         = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-secgroup.git"
  pf_prefixe     = var.pf_prefixe
  phase          = var.phase
  sg_objet       = "admin"
  sg_description = "admin"
  sg_rules = [
    {
      direction        = "ingress"
      ethertype        = "IPv4"
      protocol         = "tcp"
      port_range_min   = 22
      port_range_max   = 22

# To allow from all admin subnet
#      remote_ip_prefix = local.admin_subnet_cidr

# To allow only from Bastion
# Attention :
#  - instance_admin_ip is a list with only one ip in our case.
#  - remote_ip_prefix is appended with /32 by OpenStack. This forces replacement each time.
#           ~ remote_ip_prefix  = "172.18.0.91/32" -> "172.18.0.91" # forces replacement
#remote_ip_prefix = "${module.vm-bastion.instance_admin_ip[0]}/32"
      remote_ip_prefix = "0.0.0.0/0"
      remote_group     = ""
    },
    {
      direction        = "ingress"
      ethertype        = "IPv4"
      protocol         = "tcp"
      port_range_min   = 443
      port_range_max   = 443
      remote_ip_prefix = "0.0.0.0/0"
      remote_group     = ""
    },
  ]
}

module "sg-api" {
  source         = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-secgroup.git"
  pf_prefixe     = var.pf_prefixe
  phase          = var.phase
  sg_objet       = "api"
  sg_description = "api"
  sg_rules = [
    {
      direction        = "egress"
      ethertype        = "IPv4"
      protocol         = "tcp"
      port_range_min   = 8000
      port_range_max   = 8000
      remote_ip_prefix = "0.0.0.0/0"
      remote_group     = ""
    },
    {
      direction  = "ingress"
      ethertype = "IPv4"
      protocol = "tcp"
      port_range_min = 8000
      port_range_max = 8000
      remote_ip_prefix = "0.0.0.0/0"
      remote_group = ""
    }
  ]
}

module "sg-caddy" {
  source         = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-secgroup.git"
  pf_prefixe     = var.pf_prefixe
  phase          = var.phase
  sg_objet       = "caddy"
  sg_description = "caddy"
  sg_rules = [
    {
      direction        = "egress"
      ethertype        = "IPv4"
      protocol         = "tcp"
      port_range_min   = 80
      port_range_max   = 80
      remote_ip_prefix = "0.0.0.0/0"
      remote_group     = ""
    },
    {
      direction  = "ingress"
      ethertype = "IPv4"
      protocol = "tcp"
      port_range_min = 80
      port_range_max = 80
      remote_ip_prefix = "0.0.0.0/0"
      remote_group = ""
    },

    {
      direction        = "egress"
      ethertype        = "IPv4"
      protocol         = "tcp"
      port_range_min   = 443
      port_range_max   = 443
      remote_ip_prefix = "0.0.0.0/0"
      remote_group     = ""
    },
    {
      direction  = "ingress"
      ethertype = "IPv4"
      protocol = "tcp"
      port_range_min = 443
      port_range_max = 443
      remote_ip_prefix = "0.0.0.0/0"
      remote_group = ""
    }
  ]
}
