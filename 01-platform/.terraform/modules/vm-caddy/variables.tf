variable "server_count" {
  description = "Number of nodes to be created"
  default     = 1
}

variable "server_type" {
  description = "Type de VM"
  validation {
    condition = can(regex("^[^_]+$", var.server_type))
    error_message = "Le caractère '_' ne doit pas être présent dans server_type ?"
  }
}

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

variable "flavor_name" {
  description = "Flavor name to be used for this node"
  default     = "CO1.2"
}

variable "image_name" {
  description = "Image name to boot this node from"
  default     = "rocky8"
}

variable "metadata" {
  description = "Metadata à déposer sur les serveurs"
  default     = {}
}

variable "additional_metadata" {
  description = "Metadata additionnel"
  default     = {
  # Metadata additionnel de type key = value
  #  key1 = "value1" 
  #  key2 = "value2"
  }
}

variable "key_pair" {
  description = "Nom de la clé SSH"
}

variable "admin_network_id" {
  description = "ID du réseau d'administration"
}

variable "pub_network_id" {
  description = "Id du réseau interne"
}

variable "data_network_id" {
  description = "Id du réseau data"
}

variable "is_admin_network" {
  description = "Si = true, Instance présente sur le réseau d'administration"
  default     = false
}

variable "is_pub_network" {
  description = "Si = true, Instance présente sur le réseau de publication"
  default     = false
}

variable "is_data_network" {
  description = "Si = true, Instance présente sur le réseau de donnée"
  default     = false
}

variable "secgroup_id" {
  description = "Id of the security group to apply to this node for all ports"
  default     = []
}

variable "secgroup_name" {
  description = "Hack: name of secgroup"
  default     = []
}

variable "admin_secgroup_id" {
  description = "Id of the security group to apply to admin port of the node"
  default     = []
}

variable "pub_secgroup_id" {
  description = "Id of the security group to apply to internal port of the node"
  default     = []
}

variable "data_secgroup_id" {
  description = "Id of the security group to apply to internal port of the node"
  default     = []
}

variable "assign_admin_floating_ip" {
  description = "If true a floating IP will be attached to this node"
  default     = false
}

variable "assign_publication_floating_ip" {
  description = "If true a floating IP will be attached to this node"
  default     = false
}

variable "publication_fixed_fip" {
  description = "FIP manuelle publication"
  default     = ""
}

variable "admin_fixed_fip" {
  description = "FIP manuelle admin"
  default     = ""
}

variable "publication_fixed_vip" {
  description = "VIP manuelle publication"
  default     = ""
}

variable "admin_fixed_vip" {
  description = "VIP manuelle admin"
  default     = ""
}

variable "data_fixed_vip" {
  description = "VIP manuelle data"
  default     = ""
}

variable "publication_fixed_ips" {
  description = "Tableau des IPs manuelles publication"
  default     = [""]
}

variable "admin_fixed_ips" {
  description = "Tableau des IPs manuelles admin"
  default     = [""]
}

variable "data_fixed_ips" {
  description = "Tableau des IPs manuelles data"
  default     = [""]
}

variable "use_ha_pub" {
  description = "Vip created if Value = true "
  default     = "false"
}

variable "use_ha_admin" {
  description = "Vip created if Value = true "
  default     = "false"
}
variable "use_ha_data" {
  description = "Vip created if Value = true "
  default     = "false"
}

variable "vips_pub" {
  description = "Label des VIP Pub"
  type        = list(any)
  default     = []
}
variable "vips_admin" {
  description = "Label des VIP d'admin"
  type        = list(any)
  default     = []
}
variable "vips_data" {
  description = "Label des VIP data"
  type        = list(any)
  default     = []
}

variable "admin_floating_ip_pool" {
  description = "Name of the floating IP pool (optional if admin_fixed_fip is null)"
  default     = "FIP_ADMINISTRATION_DGFIP_PRIV"
}

variable "publication_floating_ip_pool" {
  description = "Name of the floating IP pool (optional if publication_fixed_fip is null)"
  default     = "FIP_PUBLICATION_DGFIP_PRIV"
}

variable "allowed_address_pairs" {
  description = "Liste de FIP, dans le cas de plusieurs Instance"
  default     = []
}

variable "server_group_id" {
  description = "ID d'un server_group existant pour la gestion du scheduling (seulement si server_count<=2)"
  default     = null
}

variable "servergroup_az1_id" {
  description = "ID server group AZ1"
  default     = ""
}

variable "servergroup_az2_id" {
  description = "ID server group AZ2"
  default     = ""
}



# Stockage
###################
variable "is_persistent_disk" {
  description = "Si = true, un volume persistent de taille 'disk_size' sera créé."
  default     = false
}

variable "disk_size" {
  description = "Taille du disque principal de la VM, uniquement si 'is_persistent_disk' est à true."
  default     = 10
}

variable "extra_disks" {
  description = " List of extra volumes sizes."
  default = [
 #   { Exemple de déclaration d'ajout de disque
 #     type       = ""  #disk_data
 #     device     = "" #/dev/vdb ; paramètre optionnel
 #     size       = 0
 #     mountpath  = ""  #/var/lib
 #     filesystem = ""  #ext4
 #     mnt_options = "defaults"
 #     owner = "root"
 #     group = "root"
 #     mode = "0755"
 #     swappiness = 30 # valeur de swappiness positionnée dans /proc/sys/vm/swappiness  ; paramètre optionnel
 #     setype = "_default" ; paramètre optionnel
 #   },
  ]
  validation {
    condition = alltrue([
      for elt in var.extra_disks : elt.size > 0
    ])
    error_message = "Les disques ne doivent pas avoir une taille nulle."
  }
}

