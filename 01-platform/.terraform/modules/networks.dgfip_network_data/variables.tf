variable "pf_prefixe" {
  description = "Préfixe d'identification de la plateforme"
  type        = string

  validation {
    condition     = can(regex("^[a-z0-9]{3,4}-[01][12]$", var.pf_prefixe))
    error_message = "Le préfixe doit contenir 3 ou 4 caractères alpha-numériques, du trigramme ou quadrigramme, et le n° d'instance Nubo."
  }
}

variable "phase" {
  description = "Environnement (dev, prod etc) de la plateforme"
  type        = string
 
}

variable "existing_network_id" {
  description = "Identifiant du réseau existant"
  default     = ""
}

variable "is_external_network" {
  description = "Valeur = true si le réseau est externe, false sinon"
  default     = false
}

variable "is_admin_network" {
  description = "Valeur = true si le réseau créé conrespond à un réseau d'administration"
  default     = false
}

variable "external_network_name" {
  description = "Nom du réseau externe créé"
  default     = ""
}

variable "network_type" {
  description = "Type du réseau créé" 
}

variable "subnet_cidr" {
  description = "subnet CIDR"
  default     = ""
}

variable "gateway_ip" {
  description = "adresse de la passerelle par défaut"
  default     = ""
}

variable "router_type" {
  description = "Type du routeur créé"
  default     = "" 
}

variable "existing_router_ip" {
  description = "ip fixe du routeur"
  default     = ""
}

variable "external_network_id" {
  description = "Identifiant du réseau externe créé"
  default     = "" 
}

variable "dns_nameservers" {
  description = "Ips des server DNS"
  default     = ["10.156.32.33", "10.154.59.104"]
}

variable "static_routes_map" {
  default = []
}


variable "additional_routes" {
  default = []
}

variable "is_allocation_pool" {
  description = "Si true la plage d'allocation DHCP se base sur start_pool, end_pool"
  default     = false 
}

variable "start_pool" {
  description = "Début du pool d'allocation"
  default = ""
  
}

variable "end_pool" {
  description = "Fin du pool d'allocation"
  default = ""
  
}