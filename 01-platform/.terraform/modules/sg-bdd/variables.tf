
variable "sg_description" {
  description = "Description du security group"
}

variable "sg_rules" {
  description = "Liste des règles du sécurity group"
  default     = []
}

variable "delete_default_rules" {
  description = "Booleen indiquant s'il faut supprimer les regles par defaut ou non"
  type        = bool
  default     = false
  }

variable "remote_group" {
  description = "SG source autorisé à accéder"
  default     = ""
}
variable "sg_objet" {
  description = "Élément sur lequel porte le security group"
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
