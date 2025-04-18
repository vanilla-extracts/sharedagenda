# Module Terraform : terraform-openstack-instance

## Description

Création d'une instance de VM générique pour plusieurs cas d'usage.
L'instance créée à partir de ce module  peut être attaché aux 3 réseaux DGFIP (Administration, publication et Data). Elle peut aussi avoir une FIP sur le réseau d'administration ou de publication. Elle dispose également de l'option de rattacher un volume supplémentaire avec une taille à définir.

Enfin, ou peut choisir le nombre d'instance à créer qui seront par défaut répartis sur les 2 AZs. 
Si le nombre d'instances est supérieur à 2, alors on utilise les server_group avec une notion de soft-anti-affinité (répartition des instances d'une même AZ sur des hyperviseurs différents)


## Usage

Exemple de déclaration 
```yaml
module "dgfip_calcul_instance_<example>" {
  source                         = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/calcul/terraform-openstack-instance.git?ref=<vX.X.X>"
  pf_prefixe                     = var.pf_prefixe
  phase                          = var.phase
  server_type                    = var.group
  image_name                     = var.image_name
  server_count                   = var.server_count
  flavor_name                    = var.flavor_name
  key_pair                       = var.key_pair
  admin_network_id               = var.admin_network_id
  pub_network_id                 = var.pub_network_id
  data_network_id                = var.data_network_id
  is_admin_network               = var.is_admin_network
  is_pub_network                 = var.is_pub_network
  is_data_network                = var.is_data_network
  admin_secgroup_id              = [module.dgfip_network_secgroup_<example>_admin.secgroup_id]
  pub_secgroup_id                = [module.dgfip_network_secgroup_<example>_pub.secgroup_id]
  data_secgroup_id               = [module.dgfip_network_secgroup_<example>_data.secgroup_id]
  secgroup_name                  = []
  extra_disks                    = var.extra_disks
  servergroup_az1_id             = var.servergroup_az1_id
  servergroup_az2_id             = var.servergroup_az2_id
  vips_pub                       = var.vips_pub
  vips_admin                     = var.vips_admin
  vips_data                      = var.vips_data

}
```
## Prérequis
Les Prérequis suivants sont nécéssaires 
| Nom    | Version |
| ------ | ------ |
| Terraform | ~> 1.6 |
| terraform-provider-openstack | ~> 1.53 |

## Dépendances 

Pas de dépendances

## Inputs
| Variable | Description | Obligatoire | Valeur par défaut|
| ------ | ------ | ------ | ------ |
| server_count | Nombre d'instance à créer | Oui | 1 |
| flavor_name | Nom de la flavor utilisée pour l'instance | Oui | CO1.2 |
| metadata | Metadata à déposer sur les serveurs | Non | |
| key_pair | Nom de la clé SSH | Oui | |
| image_name | Nom de l'image OS utilisée | Oui | rocky8 |
| admin_network_id | ID du réseau d'administration | Oui | |
| pub_network_id | Id du réseau interne | Non | |
| data_network_id | Id du réseau de donnée | Non | | 
| is_admin_network | Si = true, Instance présente sur le réseau d'administration | Non | False |
| is_pub_network | Si = true, Instance présente sur le réseau de publication | Non | False |
| is_data_network | Si = true, Instance présente sur le réseau de donnée | Non | False |
| use_ha_admin | Création d'une VIP pour les besoins de haute disponibilité sur le réseau ADMIN| Non | False |
| use_ha_pub | Création d'une VIP pour les besoins de haute disponibilité sur le réseau PUBLICATION | Non | False |
| use_ha_data | Création d'une VIP pour les besoins de haute disponibilité sur le réseau DATA | Non | False |
| secgroup_id | Id of the security group to apply to this node for all ports | Non | |
| secgroup_name | Hack: name of secgroup | Non | [] |
| admin_secgroup_id | Id of the security group to apply to admin port of the node | Non | 
| pub_secgroup_id | Id of the security group to apply to internal port of the node | Non | |
| data_secgroup_id | Id of the security group to apply to internal port of the node | Non | |
| assign_admin_floating_ip | If true a floating IP will be attached to this node | Non | False |
| assign_publication_floating_ip | If true a floating IP will be attached to this node | Non | False
| admin_fixed_fip | FIP manuelle admin | Non | |
| publication_fixed_fip | FIP manuelle publication | Non | |
| admin_fixed_vip | VIP manuelle admin | Non | |
| publication_fixed_vip | VIP manuelle publication | Non | |
| data_fixed_vip | VIP manuelle data | Non | |
| allowed_address_pairs | Liste de VIP autorisé dans un port | Non | |
| extra_disks | Liste des volumes (max 10). Liste vide = pas de volume | Non | [{ type = "" size = 0 mountpath = "" filesystem = "" mnt_options = "" owner = "" group = "" mode = "" swappiness = 30  setype = "_default"}] |
| pf_prefixe  |    Préfixe formé du trigramme ou quadrigramme, et du numéro d'instance Nubo | Oui | |
| server_type |    Fonction de la ressource ex :"bdd". Ne doit pas contenir "_". Sera inclus dans le nom de la vip, si présente. | Oui |
| phase       |    Environnement exemple : dev, prod, rec , prod, uat ... | Oui | |
| server_group_id | ID d'un server_group existant pour la gestion du scheduling (seulement si server_count<=2) | Non | |
| servergroup_az1_id | ID server group AZ1 | Non | |
| servergroup_az1_id | ID server group AZ1 | Non | |
| vips_pub   | Label des VIP Pub | Non | |
| vips_data  | Label des VIP data | Non | |
| vips_admin | Label des VIP admin | Non | |
| admin_fixed_ips | Liste des IPs statiques admin. A mettre dans l'ordre si plusieurs instances. | Non | [""] |
| publication_fixed_ips | Liste des IPs statiques publication. A mettre dans l'ordre si plusieurs instances. | Non | [""] |
| data_fixed_ips | Liste des IPs statiques data. A mettre dans l'ordre si plusieurs instances. | Non | [""] |
| additional_metadata | Metadatas supplémentaires | Non | {} |  |
| is_persistent_disk | Utilisation d'un disque principal persistent (taille définie avec `disk_size`) | Non | false |
| disk_size | Taille du disque principal de la VM (`is_persistent_disk` doit être à `true` pour être pris en compte) | Non | 10 |

A noter, les paramètres `swappiness` et`setype` sont optionnels (dans le bloc `extra_disks`) :
- `swappiness` sert pour le tuning système
- `setype` sert dans le cas de SELinux, sur le *filesystem* associé (valeur par défaut :`_default`)

## Outputs
| Nom | description |
|------ | ------ |
| instance_id | Identifiant de l'instance |
| instance_metadata | Metadata de l'instance | 
| instance_hostnames | Nom de l'instance |
| instance_extra_disks | Liste des disques attachés | 
| instance_admin_ip | Ip d'admin de l'instance |
| instance_pub_ip | Ip de publication de l'instance | 
| instance_data_ip | Nom de l'instance |
| instance_extra_disks | Ip de donnée de l'instance | 

## ChangeLog
[v2.3.0]
  * Possibilité d'utiliser un disque principal persistent et de définir sa taille (Issue #37) 

[v2.2.1]
  * Correction afin de pouvoir utiliser un tiret dans le `server_type`, ou `group` sur les modules dont il dépend (Issue #36)

[v2.2.0]
  * Fixer l'ordre d'attachement des disques pour éviter des problèmes lors de la reconstruction (Issue #35)<br/>
    :warning: A noter qu'il y a une rupture de compatibilité sur l'attachement des disques avec la version précédente :warning:

[v2.1.1]
  * Ajout documentaire du paramètre `swapiness` dans la variable `extra_disks` (Issue #33)
  * Ajout du paramètre optionnel`setype` dans la variable `extra_disks` (Issue #34) 

[v2.1.0]
  * Possibilité d'ajouter des metadatas supplémentaires 

[v2.0.1]
  * Correction double attribution FIP pour les VIP
  * Mise à jour du README


[v2.0.0]
  * Creation des disques via `for_each` au lieu de `count`  
  * Montée de version du provider `terraform-provider-openstack`
  * Montée de version de la CLI terraform
  * Montée de version de la ressource `openstack_blockstorage_volume_v3` vers `openstack_blockstorage_volume_v3`


[v1.2.0]
  * Possibilité de définir des VIPs statiques sur les interfaces admin, publication et data

[v1.1.0]
  * Correction du output sur les IPs d'admin, pub et data
  * Possibilité de définir des IPs statiques sur les interfaces admin, publication et data

[v1.0.8]
  * Passage en trigramme/quadrigramme pour le pf_prefixe
  * Modification de la valeur par défaut pour secgroup_name
  * Exclusion des fichiers *.tfvars dans le .gitignore
  * Mise à jour du Readme

[v1.0.7]
  * Ajout des options owner/group/mode dans extra_disks
  * Update Readme
  
[v1.0.6]
  * Ajout de mnt_options dans extra_disks
  * Mise à jour de la validation du pf_prefixe
  * Mise à jour des outputs pour un server_count =0
  
[v1.0.5]
  * Rendre libre la valeur de la propriété 'phase'
  * Ajout des servergroup 
  * Ajout des VIP en tant que metadata
  * Ajout du group/server_type dans la vip si présente

[v1.0.4]
  * Ajout d'attachement multiple de disques
  * Convention de nommage
  * Ajout d'output
  * Ajout de metadata sur les disques

[v1.0.3] Ajout d'une VIP optionnelle pour les besoins de haute disponibilité sur les réseaux ADMIN et DATA.

[v1.0.2] Ajout d'une VIP optionnelle pour les besoins de haute disponibilité sur le réseau PUBLICATION.

[v1.0.1] Mise à jour nom des résedaux externes pour la DGFIP.

[v1.0.0] Initial release

