# Bastion
# =======

# Affichage des ip admin du Bastion
output "bastion_ip_admin" {
    value = module.vm-bastion.instance_admin_ip
}

# Webservers
# ==========

# Affichage des ip admin des webservers
output "apis_ip_admin" {
    value = module.vm-apis.instance_admin_ip
}

# Affichage des ip publi des webservers
output "apis_ip_pub" {
    value = module.vm-apis.instance_pub_ip
}

# Affichage des ip publi des webservers
output "apis_ip_data" {
    value = module.vm-apis.instance_data_ip
}

# HAProxyies
# ==========

# Affichage des ip admin des haproxies
output "caddy_ip_admin" {
    value = module.vm-caddy.instance_admin_ip
}

# Affichage des ip publi des haproxies
output "caddy_ip_publi" {
    value = module.vm-caddy.instance_pub_ip
}
