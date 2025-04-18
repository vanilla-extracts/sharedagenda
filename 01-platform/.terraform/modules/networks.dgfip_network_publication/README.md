# Module Terraform : terraform-openstack-network

## Description

Création d'un réseau sur un tenant Openstack.
Le réseau créé peut être interne, externe associé au FIP de publication ou externe associé au FIP d'administration.

## Usage

Exemple de déclaration pour un réseau externe d'administration
```ruby
module "dgfip_network_admin" {
  source                = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-network.git?ref=<vX.X.X>"
  phase                 = var.phase
  pf_prefixe            = var.pf_prefixe
  external_network_name = var.admin_external_network_name
  is_external_network   = true
  is_admin_network      = true
  network_type          = local.admin_network_type
  router_type           = local.admin_router_type
  existing_network_id   = var.existing_admin_network_id
  subnet_cidr           = var.admin_subnet_cidr
  router_ip             = var.admin_gateway_ip
  gateway_ip            = var.admin_gateway_ip
  static_routes_map     = var.admin_static_routes_map
  additional_routes     = var.additional_admin_routes

}
```
Exemple de déclaration pour un réseau externe de publication 
```ruby
module "dgfip_network_publication" {
  source                = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-network.git?ref=<vX.X.X>"
  phase                 = var.phase
  pf_prefixe            = var.pf_prefixe
  external_network_name = var.pub_external_network_name
  is_external_network   = true
  is_admin_network      = false
  network_type          = local.pub_network_type
  router_type           = local.pub_router_type
  existing_network_id   = var.existing_pub_network_id
  subnet_cidr           = var.pub_subnet_cidr
  router_ip             = var.pub_gateway_ip
  gateway_ip            = var.pub_gateway_ip
  static_routes_map     = var.pub_static_routes_map
  additional_routes     = var.additional_pub_routes
}
```
Exemple de déclaration pour un réseau interne 
```ruby
module "dgfip_network_data" {
  source                = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-network.git?ref=<vX.X.X>"
  count                 = var.create_data_network ? 1 : 0
  phase                 = var.phase
  pf_prefixe            = var.pf_prefixe
  is_external_network   = false
  is_admin_network      = false
  network_type          = var.network_type
  existing_network_id   = var.existing_data_network_id
  subnet_cidr           = var.data_subnet_cidr
}
```

## Prérequis
Les Prérequis suivants sont nécéssaires 
| Nom    | Version |
| ------ | ------ |
| Terraform | ~> v1.6 |
| terraform-provider-openstack | ~> v1.53 |

## Dépendances 

Pas de dépendances

## Inputs
| Variable | Description | Obligatoire | Valeur par défaut|
| ------ | ------ | ------ | ------ |
| pf_prefixe | Préfixe formé du trigramme ou quadrigramme, et du numéro d'instance Nubo | Oui |  |
| phase | Environnement/phase du projet | Oui |  |
| network_type | Type du réseau créé (adm, pub, data)| Oui | |
| router_type | Type du routeur créé (adm, pub, data) | Oui | |
| subnet_cidr | subnet CIDR | Oui | |
| existing_network_id | Identifiant du réseau existant | Non | |
| is_external_network | Valeur = true si le réseau est externe, false sinon | Non | false |
| is_admin_network | Valeur = true si le réseau créé conrespond à un réseau d'administration | Non | false | 
| external_network_name | Nom du réseau externe créé | Non | |
| external_network_id | Identifiant du réseau externe créé | Non | |
| gateway_ip | adresse de la passerelle par défaut | Non | |
| existing_router_ip | ip fixe du routeur | Non | |
| static_routes_map | Map de route statique par plateforme NUBO '{NUBO01 = [],NUBO11 = [],NUBO02 = []}' | Non | |
| additional_routes | Tableau de routes additionnelles | Non | |
| is_allocation_pool | Si true la plage d'allocation DHCP se base sur start_pool, end_pool | Non | False |
| start_pool         | Début du pool d'allocation       | Non | |
| end_pool           | Fin du pool d'allocation         | Non | |


## Outputs
| Nom | description |
|------ | ------ |
| network_id | Identifiant du réseau créé |
| router_ip  | IP du routeur créé         |

## ChangeLog
[v2.0.1]
  * Création des DNS uniquement sur nubo 01 et 11 (Issue: #22)

[v2.0.0]
  * Changement de la version de terraform de 1.0.X à 1.6.X (Issue: #18)
  * Changement de la version du provider Openstack de 1.47.0 à 1.53.X (Issue: #18)

[v1.1.1]
  * Supression de la variable nubo 

[v1.1.0]
  * Possibilité de fixer une plage d'IP


[v1.0.7]
  * changement de nom de variable 'router_ip' vers 'existing_router_ip'
  * Externalisation de l'IP du router
  * Fusion des routes statiques

[v1.0.6]
  * Correction de plusieurs valeurs par défaut

[v1.0.5]
  * Correction de plusieurs valeurs par défaut
  * Modification en obligatoire de 3 paramètres : network_type, router_type et subnet_cidr
  * Passage en trigramme/quadrigramme pour le pf_prefixe

[v1.0.4]
  * Mise à jour de la validation du pf_prefixe

[v1.0.3] 
  * Possibilité d'ajout de routes additionnelles quelques soit le réseau externe 

[v1.0.2] 
  * Convention de nommage des ressources

[v1.0.1] 
  * Possibilité d'ajouter des routes additionnelles 

[v1.0.0] Initial release

