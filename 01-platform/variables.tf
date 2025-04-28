locals {
  # Numéro de la plateforme
  # nubo       = var.nubo
  # cloud      = var.cloud
  # pf_prefixe = var.pf_prefixe

  number_of_api_servers = 1 
  number_of_database_servers = 2 
  number_of_caddy_servers = 1 
  number_of_bastion = 1 
  # Définition des réseau IP internes (Norme DGFiP)
  admin_subnet_cidr = "172.18.0.0/24" # Réseau IP interne d'administration
  pub_subnet_cidr   = "172.16.0.0/24" # Réseau IP interne de publication
  data_subnet_cidr  = "172.14.0.0/24" # Réseau IP interne de données

  # Définition du réseau externe d'administration
  fip_administration_network_name = "FIP_ADMINISTRATION_DGFIP_PRIV"
  #fip_administration_network_ip   = var.fip_map[var.platform_id][0]

  # Définition du réseau externe de publication
  fip_publication_network_name = "FIP_PUBLICATION_DGFIP_PRIV"
  #fip_publication_network_ip   = var.fip_map[var.platform_id][1]
}

# Variables définies au niveau du shell (Ne pas modifier).
variable "cloud" { description = "Nom du cloud" }
variable "platform_id" { description = "Identifiant de la plateforme" }
variable "pf_prefixe" { description = "Préfixe commun aux ressources de la plateforme" }
variable "home_directory" { description = "Home directory" }
# Table des IP flottantes (Ne pas modifier).
variable "fip_map" {
  default = { #   FIP_ADMIN,       FIP_PUBLI
    # Projet DevOps
    "01" = ["10.125.7.199", "10.125.14.60"],
    "02" = ["10.125.4.244", "10.125.13.209"],
    "03" = ["10.125.6.248", "10.125.14.52"],
    "04" = ["10.125.6.151", "10.125.13.223"],
    "05" = ["10.125.7.143", "10.125.14.178"],
    "06" = ["10.125.4.220", "10.125.15.99"],
    "07" = ["10.125.5.78", "10.125.15.144"],
    "08" = ["10.125.6.35", "10.125.12.204"],
    "09" = ["10.125.5.39", "10.125.12.28"],
    "10" = ["10.125.5.61", "10.125.15.213"],
    "11" = ["10.125.4.57", "10.125.14.74"],
    "12" = ["10.125.4.89", "10.125.13.204"],
    "13" = ["10.125.7.126", "10.125.12.179"],
    "14" = ["10.125.4.155", "10.125.14.220"],
    "15" = ["10.125.7.226", "10.125.15.90"],
    # Projet Ludwin
    "101" = ["10.125.5.94", "100.70.0.117"],
    "102" = ["10.125.7.165", "100.70.1.78"],
    "103" = ["10.125.4.92", "100.70.1.80"],
  }
}

# Nouvelles variable pour les modules A2C
variable "nubo" { default = "10" }
variable "phase" { default = "dev" }
variable "key_pair" {}
variable "sg_description" { default = "" }
variable "image_name" { default = "debian12" }
variable "flavor_name" { default = "CO1.1" }
variable "bdd_flavor_name" { default = "CO1.2" }
