# Rôle Ansible : hosts-aliases

## Description

Gestion du fichier /etc/hosts via l'inventaire Ansible. Le fichier est rempli avec les adresses IP issues des facts Ansible.
Cela permet de palier au fait que le serveur DNS ne serait pas joignable par exemple. 

## Prérequis.

`netaddr` doit être déclaré dans le fichier des requirements pip Ansible. Le rôle `hosts-aliases` a été testé avec la version *1.2.1* de netaddr.

Exemple:
```
ansible==6.7.0
ansible-core==2.13.13
ansible-lint==6.6.1
openstacksdk==2.1.0
netaddr==1.2.1
```

## Variables par défaut

Les variables utilisées par le rôle pour compléter le fichier /etc/hosts

```yaml

nuages:
  - address: 100.67.0.14
    hostname: nuage01.dgfip.finances.rie.gouv.fr
    aliases: 
      - "nuage01"
      - "nubo01"
  - address: 100.67.0.46
    hostname: nuage11.dgfip.finances.rie.gouv.fr
    aliases: 
      - "nuage11"
      - "nubo11"
  - address: 100.67.0.110
    hostname: nuage02.dgfip.finances.rie.gouv.fr
    aliases: 
      - "nuage02"
      - "nubo02"
  - address: 100.67.0.78
    hostname: nuage12.dgfip.finances.rie.gouv.fr
    aliases: 
      - "nuage12"
      - "nubo12"

```

Suffixe par zone réseau :

```yaml
hosts_suffix:
  admin: "" # suffixe de nom de machine, sur réseau adm
  pub: "-int" # suffixe de nom de machine, sur réseau pub
  data: "-data" # suffixe de nom de machine, sur réseau data
```

## Autres Variables

Les variables `extra_hosts` et `extra_hosts_opt` permettent de définir des hosts supplémentaires, qui par exemple ne seraient pas vus par Ansible. 
On peut ainsi définir des entrées qui soient communes à l'ensemble des hosts (extra_vars) et des entrées qui soient propres à certains types de machines (extra_hosts_opt).

Exemple, à compléter en fonction de votre besoin, dans le fichier `group_vars/all.yml` pour permettre à l'ensemble des machines d'accéder à l'antivirus sans avoir une résolution DNS :

```yaml
---
extra_hosts:
  - address: 10.154.61.6
    hostname: proxy-antivirus-alternatif.infra.dgfip
  - address: 10.154.68.19
    hostname: proxy-antivirus.infra.dgfip
```

Exemple, à compléter en fonction de votre besoin, dans le fichier `group_vars/platform_bastion.yml` pour permettre aux bastions d'accéder aux API sans avoir une résolution DNS :

```yaml
---
extra_hosts_opt:
  - address: 100.67.0.46
    hostname: nuage11.dgfip.finances.rie.gouv.fr
    aliases: 
      - "nuage11"
      - "nubo11"
      - "nubo11-api"
```

Le dossier inventories contient les inventaires des différents environnements (dans notre exemple, on affiche un seul environnement: dev).

```yaml
├ inventories
│ ├── common
│ │   ├── localhost.yml
│ │   └── group_vars
│ │       ├── all.yml
│ │       ├── platform_bastion.yml
│ │       └── protected_nodes.yml
│ └── dev
│     ├── openstack.yml
│     ├── platform.yml
```

- common/group_vars/all.yml 
Le fichier all.yml contient les variables suivantes:

```yaml
# réseaux par défaut 

admin_network_name: "{{ deploy.pf_prefixe }}-network-adm-{{ deploy.phase }}"     # nom du réseau d'administration
pub_network_name: "{{ deploy.pf_prefixe }}-network-pub-{{ deploy.phase }}"  # nom du réseau de publication
data_network_name: "{{ deploy.pf_prefixe }}-network-data-{{ deploy.phase }}"     # nom du réseau data

data_ip: "{{ openstack.addresses[data_network_name].0.addr }}"                   # Adresse IP réseau data.
admin_ip: "{{ openstack.addresses[admin_network_name].0.addr }}"
pub_ip: "{{ openstack.addresses[pub_network_name].0.addr }}"

ansible_user: cloudadm
cloud: "{{ deploy.cloud_name }}"
```

- common/group_vars/protected_nodes.yml contient la configuration ssh pour toutes les machines protected_nodes, en effet, l'accès à ces machines se fait par rebond sur le bastion

```yaml
bastion_host: "{{ groups.platform_bastion.0 | default(omit) }}"
ansible_ssh_common_args: >-
-o ForwardAgent=yes
-o StrictHostKeyChecking=no
{% with bastion = hostvars[bastion_host] %}
-o Hostname={{ openstack.addresses[admin_network_name].0.addr }}
-o "ProxyCommand ssh -o \"StrictHostKeyChecking no\" {{ ansible_user }}@{{ bastion.ansible_host }} -W %h:%p"
{%- endwith %}
```

## CHANGELOG

[v1.1.2]
 * Correctif documentaire (issue #8)
 
[v1.1.1]
 * Correctifs du template hosts (issue #7)

[v1.1.0] 
 * Ajout de la variable `extra_hosts_opt` pour permettre des fichiers hosts adaptés (issue #6 ).

[v1.0.1] 
 * Ajout des prérequis dans le README (issue #5 ).

[v1.0.0]
 * Correction du problème des lignes dupliquées (issue #2).
 * Version initiale.