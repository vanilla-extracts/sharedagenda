data "openstack_networking_subnet_v2" "admin_subnet" {
  count      = var.is_admin_network ? 1 : 0
  network_id = var.admin_network_id
}

data "openstack_networking_subnet_v2" "pub_subnet" {
  count      = var.is_pub_network ? 1 : 0
  network_id = var.pub_network_id
}

data "openstack_networking_subnet_v2" "data_subnet" {
  count      = var.is_data_network ? 1 : 0
  network_id = var.data_network_id
}

data "openstack_images_image_v2" "image" {
  count     = var.is_persistent_disk ? 1 : 0
  name      = var.image_name
  most_recent = true
}

# create port for admin network
resource "openstack_networking_port_v2" "admin_network_port" {
  count              = var.is_admin_network ? var.server_count : 0
  name               = format("%s-port-adm-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  network_id         = var.admin_network_id
  admin_state_up     = "true"
  security_group_ids = concat(var.secgroup_name, var.admin_secgroup_id)
  fixed_ip {
    subnet_id = data.openstack_networking_subnet_v2.admin_subnet.0.id
    ip_address = length(var.admin_fixed_ips) > count.index ? var.admin_fixed_ips[count.index] : null
  }

  dynamic "allowed_address_pairs" {
    for_each = var.use_ha_admin ? [true] : []
    content {
      ip_address = openstack_networking_port_v2.vip_admin_network_port.0.all_fixed_ips[0]
    }
  }
  dynamic "allowed_address_pairs" {
    for_each = toset(var.allowed_address_pairs)
    content {
      ip_address = allowed_address_pairs.key
    }
  }
}

# create port for publication network
resource "openstack_networking_port_v2" "pub_network_port" {
  count              = var.is_pub_network ? var.server_count : 0
  name               = format("%s-port-pub-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  network_id         = var.pub_network_id
  admin_state_up     = "true"
  security_group_ids = concat(var.secgroup_name, var.pub_secgroup_id)
  fixed_ip {
    subnet_id = data.openstack_networking_subnet_v2.pub_subnet.0.id
    ip_address = length(var.publication_fixed_ips) > count.index ? var.publication_fixed_ips[count.index] : null
  }
  dynamic "allowed_address_pairs" {
    for_each = var.use_ha_pub ? [true] : []
    content {
      ip_address = openstack_networking_port_v2.vip_pub_network_port.0.all_fixed_ips[0]
    }
  }
  dynamic "allowed_address_pairs" {
    for_each = toset(var.allowed_address_pairs)
    content {
      ip_address = allowed_address_pairs.key
    }
  }
}

# create port for data network
resource "openstack_networking_port_v2" "data_network_port" {
  count              = var.is_data_network ? var.server_count : 0
  name               = format("%s-port-data-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  network_id         = var.data_network_id
  admin_state_up     = "true"
  security_group_ids = concat(var.secgroup_name, var.data_secgroup_id)
  fixed_ip {
    subnet_id = data.openstack_networking_subnet_v2.data_subnet.0.id
    ip_address = length(var.data_fixed_ips) > count.index ? var.data_fixed_ips[count.index] : null
  }
  dynamic "allowed_address_pairs" {
    for_each = var.use_ha_data ? [true] : []
    content {
      ip_address = openstack_networking_port_v2.vip_data_network_port.0.all_fixed_ips[0]
    }
  }
  dynamic "allowed_address_pairs" {
    for_each = toset(var.allowed_address_pairs)
    content {
      ip_address = allowed_address_pairs.key
    }
  }
}

# Add VIP port for admin network if use_ha_admin == true
resource "openstack_networking_port_v2" "vip_admin_network_port" {
  count          = var.use_ha_admin ? 1 : 0
  name           = format("%s-port-adm-vip-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  network_id     = var.admin_network_id
  admin_state_up = "true"
  fixed_ip {
    subnet_id = data.openstack_networking_subnet_v2.admin_subnet.0.id
    ip_address = var.admin_fixed_vip != "" ? var.admin_fixed_vip : null
  }
}

# Add VIP port for publication network if use_ha_pub == true
resource "openstack_networking_port_v2" "vip_pub_network_port" {
  count          = var.use_ha_pub ? 1 : 0
  name           = format("%s-port-pub-vip-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  network_id     = var.pub_network_id
  admin_state_up = "true"
  fixed_ip {
    subnet_id = data.openstack_networking_subnet_v2.pub_subnet.0.id
    ip_address = var.publication_fixed_vip != "" ? var.publication_fixed_vip : null
  }
}

# Add VIP port form data network if use_ha_data == true
resource "openstack_networking_port_v2" "vip_data_network_port" {
  count          = var.use_ha_data ? 1 : 0
  name           = format("%s-port-data-vip-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  network_id     = var.data_network_id
  admin_state_up = "true"
  fixed_ip {
    subnet_id = data.openstack_networking_subnet_v2.data_subnet.0.id
    ip_address = var.data_fixed_vip != "" ? var.data_fixed_vip : null
  }
}

# Construction de l'objet metadata à partir d'une fusion de la liste des noms de vip passés dans la variable "vips_*" et de la liste des ports créés  
locals {
  metadata_pub         = var.use_ha_pub ? zipmap(var.vips_pub, openstack_networking_port_v2.vip_pub_network_port[*].all_fixed_ips[0]) : null
  metadata_admin       = var.use_ha_admin ? zipmap(var.vips_admin, openstack_networking_port_v2.vip_admin_network_port[*].all_fixed_ips[0]) : null
  metadata_data        = var.use_ha_data ? zipmap(var.vips_data, openstack_networking_port_v2.vip_data_network_port[*].all_fixed_ips[0]) : null 
  metadata = merge(local.metadata_pub, local.metadata_admin, local.metadata_data, var.additional_metadata, var.metadata,)
} 

resource "openstack_compute_instance_v2" "instance" {
  count             = var.server_count
  name              = format("%s-vm-%s-%02d-%s", var.pf_prefixe, var.server_type, count.index + 1, var.phase)
  image_name        = var.image_name
  flavor_name       = var.flavor_name
  key_pair          = var.key_pair
  availability_zone = "AZ${format("%01d", ((count.index + 1) % 2) + 1)}"

  dynamic "block_device" {
    for_each = (var.is_persistent_disk) ? [true] : []
    content {
      source_type           = "image"
      uuid                  = data.openstack_images_image_v2.image.0.id
      destination_type      = "volume"
      boot_index            = 0
      volume_size           = var.disk_size
      delete_on_termination = true
    }
  }

  dynamic "network" {
    for_each = (var.is_admin_network) ? [true] : []
    content {
      port = element(openstack_networking_port_v2.admin_network_port.*.id, count.index)
    }
  }

  dynamic "network" {
    for_each = var.is_pub_network ? [true] : []
    content {
      port = element(openstack_networking_port_v2.pub_network_port.*.id, count.index)
    }
  }

  dynamic "network" {
    for_each = var.is_data_network ? [true] : []
    content {
      port = element(openstack_networking_port_v2.data_network_port.*.id, count.index)
    }
  }
  metadata = local.metadata

  lifecycle {
    ignore_changes = [key_pair, image_name, block_device]
  }

  dynamic "scheduler_hints" {
    for_each = var.server_count > 2 || var.server_group_id != null ? [1] : []
    content {
      group = var.server_count > 2 ? element([var.servergroup_az1_id, var.servergroup_az2_id], count.index) : var.server_group_id
    }
  }

}

resource "openstack_networking_floatingip_v2" "admin_floating_ip" {
  count = (var.assign_admin_floating_ip && var.admin_fixed_fip == "") ? (var.is_admin_network ? var.server_count : 0) : 0
  pool  = var.admin_floating_ip_pool
}
resource "openstack_networking_floatingip_associate_v2" "associate_admin_floating_ip" {
  count       = var.assign_admin_floating_ip ? (var.is_admin_network ? (var.use_ha_admin ? 1 : var.server_count): 0) : 0
  floating_ip = var.admin_fixed_fip != "" ? element(var.admin_fixed_fip, count.index) : element(openstack_networking_floatingip_v2.admin_floating_ip.*.address, count.index)
  port_id     = var.use_ha_admin ? openstack_networking_port_v2.vip_admin_network_port.0.id : element(openstack_networking_port_v2.admin_network_port.*.id, count.index)
  fixed_ip    = var.use_ha_admin ? openstack_networking_port_v2.vip_admin_network_port.0.all_fixed_ips[0] : element(openstack_networking_port_v2.admin_network_port, count.index).all_fixed_ips[0]
  depends_on  = [openstack_compute_instance_v2.instance]
}


resource "openstack_networking_floatingip_v2" "publication_floating_ip" {
  count = (var.assign_publication_floating_ip && var.publication_fixed_fip == "") ? (var.is_pub_network ? var.server_count : 0) : 0
  pool  = var.publication_floating_ip_pool
}

resource "openstack_networking_floatingip_associate_v2" "associate_pub_floating_ip" {
  count       = var.assign_publication_floating_ip ? (var.is_pub_network ? (var.use_ha_pub ? 1 : var.server_count): 0) : 0
  floating_ip = var.publication_fixed_fip != "" ? element(var.publication_fixed_fip, count.index) : element(openstack_networking_floatingip_v2.publication_floating_ip.*.address, count.index)
  port_id     = var.use_ha_pub ? openstack_networking_port_v2.vip_pub_network_port.0.id : element(openstack_networking_port_v2.pub_network_port.*.id, count.index)
  fixed_ip    = var.use_ha_pub ? openstack_networking_port_v2.vip_pub_network_port.0.all_fixed_ips[0] : element(openstack_networking_port_v2.pub_network_port, count.index).all_fixed_ips[0]
  depends_on  = [openstack_compute_instance_v2.instance]
}

locals {
  disks =merge( [
    for key,value in var.extra_disks : {
      for i in range(1,var.server_count+1) :
        format("%s-vol-%s-%02d-%.03d-%s", var.pf_prefixe, var.server_type, i , key+1, var.phase) => {
          server_name   = element(openstack_compute_instance_v2.instance.*.name, i -1 ),
          server_id     = element(openstack_compute_instance_v2.instance.*.id, i -1 ),
          metadata = {
            type        = value.type
            size        = value.size
            mountpath   = value.mountpath
            filesystem  = value.filesystem
            mnt_options = value.mnt_options
            owner       = value.owner
            group       = value.group
            mode        = value.mode
            swappiness  = try(value.swappiness, null)
            setype      = try(value.setype, "_default")
          }
        }
    }
  ]...)

  volumes=distinct([
    for key,value in local.disks :
        regex("[0-9][0-9][0-9]",key)
  ])

  attachments =flatten([
    for vol in local.volumes : {
      for key,value in local.disks :
        regex("[0-9][0-9][0-9]",key) => {
          server_name = value.server_name
          server_id   = value.server_id
          volume_name = key
          volume_id   = openstack_blockstorage_volume_v3.extra_disk[key].id
        }... if regex("[0-9][0-9][0-9]",key) == vol
    }
  ])
}

resource "openstack_blockstorage_volume_v3" "extra_disk" {
  for_each = local.disks
  name = each.key
  size = each.value.metadata.size
  metadata = each.value.metadata
}

########################################################################################################################################
# Attaching disk by disk on all instances, with "depends_on" argument (Trick to be sure that volumes are attached in the right order)
# Of course it makes a lot of resources, but only used if disks declared into "extra_disks" variable (max 10 disks per instance)
########################################################################################################################################
resource "openstack_compute_volume_attach_v2" "attach_extra_disk_1" {
  for_each = length(local.attachments) > 0 ? { for key,value in local.attachments[0]["001"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_2" {
  for_each = length(local.attachments) > 1 ? { for key,value in local.attachments[1]["002"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_1]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_3" {
  for_each = length(local.attachments) > 2 ? { for key,value in local.attachments[2]["003"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_2]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_4" {
  for_each = length(local.attachments) > 3 ? { for key,value in local.attachments[3]["004"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_3]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_5" {
  for_each = length(local.attachments) > 4 ? { for key,value in local.attachments[4]["005"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_4]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_6" {
  for_each = length(local.attachments) > 5 ? { for key,value in local.attachments[5]["006"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_5]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_7" {
  for_each = length(local.attachments) > 6 ? { for key,value in local.attachments[6]["007"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_6]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_8" {
  for_each = length(local.attachments) > 7 ? { for key,value in local.attachments[7]["008"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_7]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_9" {
  for_each = length(local.attachments) > 8 ? { for key,value in local.attachments[8]["009"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_8]
}

resource "openstack_compute_volume_attach_v2" "attach_extra_disk_10" {
  for_each = length(local.attachments) > 9 ? { for key,value in local.attachments[9]["010"]: key => value } : {}
  instance_id = each.value.server_id
  volume_id   =  each.value.volume_id

  depends_on = [openstack_compute_volume_attach_v2.attach_extra_disk_9]
}
########################################################################################################################################