# Module Terraform : terraform-openstack-secgroup

## Description

Création d'un Security Group avec ses règles.

## Usage
Example de déclaration du module terraform-openstack-secgroup.

```ruby
module "dgfip_network_secgroup_example" {
  source         = "git::https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/module-terraform-dgfip/networking/terraform-openstack-secgroup.git?ref=<vX.X.X>"
  sg_objet       = "example"
  pf_prefixe     = var.pf_prefixe
  phase          = var.phase
  sg_description = var.sg_description
  sg_rules       = var.sg_<example>_rules
  delete_default_rules = var.delete_default_rules

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
| Variable    | Description | Obligatoire | Valeur par défaut|
| ------      | ------      | ------      | ------ |
| pf_prefixe  | Préfixe formé du trigramme ou quadrigramme, et du numéro d'instance Nubo | Oui | |
| sg_objet    | Element sur lequel port le security group exemple bastion,app, haproxy | Oui || 
| phase       |    Environnement (dev, prod etc) de la plateforme | Oui || 
| sg_description | Description du security group | Non | |
| sg_rules | Liste des régles du security group | Oui | {direction = "ingress" ethertype = "IPv4" protocol = "tcp" port_range_min   = 22 port_range_max   = 22 remote_group = "" } |
| delete_default_rules | Booleen indiquant s'il faut supprimer les regles par defaut ou non | Non | false |


## Outputs
| Nom | description |
|------ | ------ |
| secgroup_name | Nom du security group crée |
| secgroup_id | Identifiant du security group crée | 

## ChangeLog
[v2.0.0]
 * Montée de version du provider 'terraform-provider-openstack'

[v1.0.4] 
 * Ajout du support trigramme/quadrigramme

[v1.0.3] 
 * Mise à jour de la validation du pf_prefixe

[v1.0.2] 
 * Implementation des remote_group

[v1.0.1] 
 * Convention nommage

[v1.0.0] 
 * Initial release
