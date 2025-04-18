data "openstack_networking_network_v2" "external_network" {
  count = var.is_external_network ? 1 : 0
  name  = var.external_network_name
}

resource "openstack_networking_network_v2" "created" {
  count = var.existing_network_id == "" ? 1 : 0
  name  = "${var.pf_prefixe}-network-${var.network_type}-${var.phase}"
}

locals {
  routes = (var.static_routes_map != "" && var.is_external_network) ? distinct(concat(var.static_routes_map, var.additional_routes)) : var.additional_routes
  pf_nubo = substr(var.pf_prefixe, -2, -1)
}


resource "openstack_networking_subnet_v2" "created" {
  count           = var.existing_network_id == "" ? 1 : 0
  name            = "${var.pf_prefixe}-subnet-${var.network_type}-${var.phase}"
  network_id      = openstack_networking_network_v2.created[0].id
  cidr            = var.subnet_cidr
  ip_version      = 4
  dns_nameservers = (local.pf_nubo == "01"||  local.pf_nubo == "11")  ?  compact(var.dns_nameservers) :[] 
  enable_dhcp     = true
  no_gateway      = (var.is_external_network && ! var.is_admin_network) ? null : true
  gateway_ip      = (var.is_external_network && ! var.is_admin_network) ? var.gateway_ip : null
  dynamic "allocation_pool" {
    for_each = var.is_allocation_pool ? [true] : []
    content {
      start = var.start_pool
      end   = var.end_pool
    }
 
  }
}

resource "openstack_networking_port_v2" "created" {
  count          = (var.is_external_network && var.existing_network_id == "") ? 1 : 0
  name           = "${var.pf_prefixe}-port-${var.network_type}-router-${var.phase}"
  network_id     = openstack_networking_network_v2.created[0].id
  admin_state_up = true
  fixed_ip {
    subnet_id  = openstack_networking_subnet_v2.created[0].id
    ip_address = var.existing_router_ip
  }
}

resource "openstack_networking_subnet_route_v2" "created" {
  count            = (var.is_external_network) ? length(local.routes) : 0
  subnet_id        = openstack_networking_subnet_v2.created[0].id
  destination_cidr = local.routes[count.index]
  next_hop         = var.gateway_ip
}

resource "openstack_networking_router_v2" "created" {
  # create only if var.existing_network_id is empty
  count               = (var.is_external_network && var.existing_network_id == "") ? 1 : 0
  name                = "${var.pf_prefixe}-router-${var.router_type}-${var.phase}"
  external_network_id = data.openstack_networking_network_v2.external_network.0.id
}

resource "openstack_networking_router_interface_v2" "created" {
  count     = (var.is_external_network && var.existing_network_id == "") ? 1 : 0
  router_id = openstack_networking_router_v2.created[0].id
  port_id   = openstack_networking_port_v2.created[0].id
}

