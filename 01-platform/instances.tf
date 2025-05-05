module "vm-bastion" {
  source      = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/s-curit/terraform-openstack-bastion.git"
  pf_prefixe  = var.pf_prefixe
  flavor_name = var.flavor_name
  image_name  = var.image_name
  key_pair    = openstack_compute_keypair_v2.ssh_keypair.name
  extra_disks = []

  admin_network_id = join("", module.networks.*.admin_network_id)
  admin_fixed_fip  = [data.openstack_networking_floatingip_v2.fip_admin.address]

  admin_sg_bastion_rules = [
    {
      direction        = "ingress"
      ethertype        = "IPv4"
      protocol         = "tcp"
      port_range_min   = 22
      port_range_max   = 22
      remote_ip_prefix = "0.0.0.0/0"
      remote_group     = ""
    },
    {
      direction        = "ingress"
      ethertype        = "IPv4"
      protocol         = "udp"
      port_range_min   = 123
      port_range_max   = 123
      remote_ip_prefix = local.admin_subnet_cidr
      remote_group     = ""
    },
    {
      direction        = "ingress"
      ethertype        = "IPv4"
      protocol         = "udp"
      port_range_min   = 514
      port_range_max   = 514
      remote_ip_prefix = local.admin_subnet_cidr
      remote_group     = ""
    }
  ]
  group = "bastion"
  additional_bastion_metadata = {
    group      = "bastion",
    pf_prefixe = "${var.pf_prefixe}"
  }
}

module "vm-apis" {
  source = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/calcul/terraform-openstack-instance.git"

  pf_prefixe   = var.pf_prefixe
  phase        = var.phase
  server_type  = "apis"
  image_name   = var.image_name
  flavor_name  = var.flavor_name
  key_pair     = openstack_compute_keypair_v2.ssh_keypair.name
  server_count = local.number_of_api_servers

  # Les trois id réseaux sont obligatoires
  admin_network_id = module.networks.admin_network_id
  pub_network_id   = module.networks.pub_network_id
  data_network_id  = module.networks.data_network_id

  # Ensuite on gère les connexions nécessaires
  is_admin_network = true
  is_pub_network   = true
  is_data_network  = true

  admin_secgroup_id = [module.sg-admin.secgroup_id]
  pub_secgroup_id   = [module.sg-api.secgroup_id]
  data_secgroup_id  = []

  metadata = {
    pf_prefixe = var.pf_prefixe
    group      = "apis"
    phase      = var.phase
  }
}

module "vm-caddy" {
  source = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/calcul/terraform-openstack-instance.git"

  pf_prefixe   = var.pf_prefixe
  phase        = var.phase
  server_type  = "caddy"
  image_name   = var.image_name
  flavor_name  = var.flavor_name
  key_pair     = openstack_compute_keypair_v2.ssh_keypair.name
  server_count = local.number_of_caddy_servers

  # Les trois id réseaux sont obligatoires
  admin_network_id = module.networks.admin_network_id
  pub_network_id   = module.networks.pub_network_id
  data_network_id  = module.networks.data_network_id

  # Ensuite on gère les connexions nécessaires
  is_admin_network = true
  is_pub_network   = true
  is_data_network  = false

  admin_secgroup_id = [module.sg-admin.secgroup_id]
  pub_secgroup_id   = [module.sg-api.secgroup_id, module.sg-caddy.secgroup_id]
  data_secgroup_id  = []

  # Ajouté pour connecter la FIP Publication au HAProxy
  publication_fixed_fip          = [data.openstack_networking_floatingip_v2.fip_publi.address]
  assign_publication_floating_ip = true

  metadata = {
    pf_prefixe = var.pf_prefixe
    group      = "caddy"
    phase      = var.phase
  }
}

module "vm-bdds" {
  source            = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/base-de-donn-es/terraform-openstack-bdd-ha.git"
  pf_prefixe        = var.pf_prefixe
  bdd_server_count  = local.number_of_database_servers
  bdd_flavor_name   = var.bdd_flavor_name
  key_pair          = openstack_compute_keypair_v2.ssh_keypair.name
  admin_network_id  = module.networks.admin_network_id
  data_network_id   = module.networks.data_network_id
  etcd_server_count = 0
  bdd_image_name    = var.image_name
  additional_bdd_metadata = {
    pf_prefixe = var.pf_prefixe
    group      = "bdds"
    phase      = var.phase
  }
}

module "vm-grafana" {
  source                      = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/observabilit/terraform-openstack-monitoring.git"
  pf_prefixe                  = var.pf_prefixe
  image_name                  = var.image_name
  phase                       = var.phase
  key_pair                    = openstack_compute_keypair_v2.ssh_keypair.name
  admin_network_id            = module.networks.admin_network_id
  pub_network_id              = module.networks.pub_network_id
  data_network_id             = module.networks.data_network_id
  additional_pub_secgroup_ids = [module.sg-grafana.secgroup_id]
  assign_admin_floating_ip    = false
  additional_monitoring_metadata = {
    pf_prefixe = var.pf_prefixe
    group      = "grafana"
    phase      = var.phase
  }
}
