# Réseau à la norme DGFiP
module "networks" {
  source        = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-networks.git"

  # Le préfixe doit contenir 3 ou 4 caractères alpha-numériques, du trigramme ou quadrigramme, et le n° d'instance Nubo.
  pf_prefixe          = var.pf_prefixe
}
