# Ansible Role : keepalived

## Description

Role d'installation du composant Keepalived.<br> 
Keepalived a pour but de surveiller l’état de cluster de type haproxy, bdd … et réaliser la bascule dans le cas d’une indisponibilité du nœud principale.<br> 
Au préalable avant l’installation de ce rôle, il faut vérifier l’installation du cluster à surveiller.<br>
Attention à mettre le chemin complet pour les binaires exécutés dans ce script et à spécifier la liste des binaires dans la variable **keepalived_check_binaries**, pour les autoriser dans les règles SElinux (si nécessaire).<br>
Le rôle permet de configurer le keepalived pour l'écoute via :
  * L’exécution d'un script
  * L'écoute sur une interface réseau
  * L'écoute d'un process
  * La présence d'un fichier

## Variables

Toutes les variables présentes dans le fichier [defaults/main.yml](defaults/main.yml) peuvent être surchargées. Une description des variables principale dans le tableau suivant : 

| Name                                             | Default Value                                                | Description                                                                  |
|--------------------------------------------------|--------------------------------------------------------------|------------------------------------------------------------------------------|
| keepalived_vrrp_net_interface     | eth1  | Interface réseau utilisée par Keepalived |
| keepalived_vip_name               |       | Nom de la VIP utilisé exemple acc-01-pub-vip-01-dev |
| keepalived_check_binaries         |       | Binaires présents dans le script de vérification à autoriser dans les politiques de sécurité SElinux |
| keepalived_check_primary_script   |       | Script de vérification utilisé pour la bascule automatique de la VIP  |
| keepalived_track_process          |       | Process à surveiler pour la bascule de la VIP                         |   
| cloud                             |       | Nom du tenant Openstack sur lequel la recherche de vip sera effectuée |
| keepalived_debian_prereq_packages | ["psmisc"] | Pour debian, installer d'éventuels packages prérequis (par défaut, `psmisc` apporte `killall`) |

:warning: **IMPORTANT** : Sur les environnements Debian, les interfaces réseaux sont généralement appelées `ens*` ; il convient d'adapter le paramètre `keepalived_vrrp_net_interface`.

## Example

### Playbook

#### Cas d'un script de verification avec killall

```yaml
- hosts: platform_haproxy
  vars:
    cloud: "a2c-dev"
    keepalived_vrrp_net_interface: "eth1"
    keepalived_vip_name: "acc-01-pub-vip-01-dev"
    keepalived_check_primary_script: "/bin/killall -0 haproxy"
    keepalived_instances:
      internal:
        interface: "{{ keepalived_vrrp_net_interface }}"
        state: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('MASTER','BACKUP')}}"
        virtual_router_id: 42
        priority: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('100','99')}}"
        vips:
          - "{{ keepalived_vip }}/24 dev {{ keepalived_vrrp_net_interface }} label {{ keepalived_vrrp_net_interface }}:1"
        track_scripts:
          - check_primary_script
    keepalived_scripts:
      check_primary_script:
        # Here is an example with a command instead of a script.
        # Add src_check_script if you want to run a script instead of a command
        # and upload it from your deploy host
        check_script: "{{ keepalived_check_primary_script }}"
        interval: 2
        weight: 2
        timeout: 2
  roles:
    - dgfip.keepalived

```

#### Cas d'un script de verification avec curl
```yaml
- hosts: platform_haproxy
  vars:
    cloud: "a2c-dev"
    use_ssl: false
    keepalived_vrrp_net_interface: "eth1"
    keepalived_vip_name: "acc-01-pub-vip-01-dev"
    keepalived_check_cmd: "/usr/bin/curl {{ (use_ssl is defined and use_ssl == true) | ternary ('-k','') }} -X GET -I --fail"
    keepalived_check_primary_script: "{{ keepalived_check_cmd }} {{ (use_ssl is defined and use_ssl == true) | ternary ('https','http') }}://<pub_ip>:8008/primary"
    keepalived_check_binaries: ["curl"]
    keepalived_instances:
      internal:
        interface: "{{ keepalived_vrrp_net_interface }}"
        state: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('MASTER','BACKUP')}}"
        virtual_router_id: 42
        priority: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('100','99')}}"
        vips:
          - "{{ keepalived_vip }}/24 dev {{ keepalived_vrrp_net_interface }} label {{ keepalived_vrrp_net_interface }}:1"
        track_scripts:
          - check_primary_script
    keepalived_scripts:
      check_primary_script:
        check_script: "{{ keepalived_check_primary_script }}"
        interval: 2
        fall: 3
        rise: 6
        weight: 2
        timeout: 2
  roles:
    - dgfip.keepalived
```
### Cas d'un script de verification plus complexe
```yaml
keepalived_vip_name: "{{ deploy.pf_prefixe }}-port-data-vip-bdd-01-{{ deploy.phase }}"
keepalived_vrrp_net_interface: "eth1"

keepalived_instances:
  internal:
    interface: "{{ keepalived_vrrp_net_interface }}"
    state: "{{((play_hosts|sort).index(inventory_hostname) == 0) | ternary('MASTER','BACKUP')}}"
    virtual_router_id: 42
    priority: "{{((play_hosts|sort).index(inventory_hostname) == 0) | ternary('100','99')}}"
    vips:
      - "{{ keepalived_vip }}/24 dev {{ keepalived_vrrp_net_interface }} label {{ keepalived_vrrp_net_interface }}:1"
    track_scripts:
      - check_primary_script
keepalived_scripts:
  check_primary_script:

    check_script: "{{ keepalived_check_primary_script }}"
    interval: 2
    weight: 2
    timeout: 2
    user: "postgres"
keepalived_global_defs:
  - router_id {{ ansible_hostname }}
  - enable_script_security
  - script_user postgres
keepalived_check_primary_script: "{{ keepalived_check_primary_script_file.path }}/{{ keepalived_check_primary_script_file.filename }}"  
keepalived_scripts:
  check_primary_script:
    check_script: "{{ keepalived_check_primary_script }}"
    interval: 2
    weight: 2
    timeout: 2
    user: "{{keepalived_check_primary_script_file.user }} {{ keepalived_check_primary_script_file.group }}" 

##################################################
keepalived_check_primary_script_file:
  user: "postgres"
  group: "postgres"
  path: "/usr/local/bin"
  filename: "check_primaty_script.sh"
  content: | # exemple script de check avant bascule
    #!/bin/bash
    # Connexion à la base de données et vérification du mode lecture seule
    IS_READONLY=$(psql -tAc "SHOW default_transaction_read_only;")
    #SHOW default_transaction_read_only retourne 'on' pour si la base est en lecture seule
    #SHOW default_transaction_read_only retourne 'off' pour si la base n'est pas en lecture seule
    if [ "$IS_READONLY" = "off" ]; then
        echo "Database is writable."
        exit 0  # Tout va bien, Keepalived ne basculera pas
    else
        echo "Database is read only or error (psql not installed)."
        exit 1 # Bascule
    fi
# #############################################

```

#### Cas d'un track process 
```yaml
- hosts: platform_haproxy
  vars:
    cloud: "a2c-dev"
    keepalived_vrrp_net_interface: "eth1"
    keepalived_vip_name: "acc-01-pub-vip-01-dev"
    keepalived_track_process: "haproxy"
    keepalived_instances:
      internal:
        interface: "{{ keepalived_vrrp_net_interface }}"
        state: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('MASTER','BACKUP')}}"
        virtual_router_id: 42
        priority: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('100','99')}}"
        vips:
          - "{{ keepalived_vip }}/24 dev {{ keepalived_vrrp_net_interface }} label {{ keepalived_vrrp_net_interface }}:1"
        track_processes:
          - haproxy_track_process
    keepalived_track_processes:
      haproxy_track_process:
        track_process: "{{ keepalived_track_process }}"
        delay: 2
        weight: 2
  roles:
    - dgfip.keepalived
```

### Cas d'un track process et d'une configuration manuelle SELinux
```yaml
- hosts: platform_haproxy
  vars:
    cloud: "a2c-dev"
    keepalived_vrrp_net_interface: "eth1"
    keepalived_vip_name: "acc-01-pub-vip-01-dev"
    keepalived_track_process: "haproxy"
    keepalived_instances:
      internal:
        interface: "{{ keepalived_vrrp_net_interface }}"
        state: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('MASTER','BACKUP')}}"
        virtual_router_id: 42
        priority: "{{(play_hosts.index(inventory_hostname) == 0) | ternary('100','99')}}"
        vips:
          - "{{ keepalived_vip }}/24 dev {{ keepalived_vrrp_net_interface }} label {{ keepalived_vrrp_net_interface }}:1"
        track_processes:
          - haproxy_track_process
    keepalived_track_processes:
      haproxy_track_process:
        track_process: "{{ keepalived_track_process }}"
        delay: 2
        weight: 2
    keepalived_selinux_automatic: false # si true,active la configuration SELinux à partir de ausearch et audit2allow. Sinon... 
    keepalived_selinux_module: "psql"
    keepalived_selinux_content: |
      module psql 1.0;
      require {
        type keepalived_t;
        type postgresql_var_run_t;
        type postgresql_db_t;
        type postgresql_etc_t;
        class dir search;
        class file { getattr open read };
        class sock_file write;
      }
      #============= keepalived_t ==============
      allow keepalived_t postgresql_db_t:dir search;
      allow keepalived_t postgresql_db_t:file open;
      allow keepalived_t postgresql_db_t:file { getattr read };
      allow keepalived_t postgresql_etc_t:dir search;
      allow keepalived_t postgresql_var_run_t:sock_file write; 
  roles:
    - dgfip.keepalived
```

### Cas d'une configuration réseau sur environement Debian
```yaml
- hosts: platform_haproxy
  vars:
    [...]
    keepalived_vrrp_net_interface: "ens3"
    [...]
  roles:
    - dgfip.keepalived
```

## ChangeLog
[v1.3.4]
* Ajout de la possibilité d'utiliser un script pour les verification avant la bascule. (Issue #18)

[v1.3.3]
* Correctif d'idempotence SELinux (Issue #16)

[v1.3.2]
* Correctif SELinux (Issue #15)

[v1.3.1]
* Correctif de code de SELinux en mode automatique (Issue #14)

[v1.3.0] 
*  Amélioration du support SELinux (choix mode automatique ou passage par le contenu d'un `.te` fourni par le projet). Par défaut,`keepalived_selinux_automati` (true) induit un comportment automatique selon la commande passée via `keepalived_check_binaries`. 

[v1.2.0]
* Support debian 12 (Issue #12)

[v1.1.3]
* Correction d'une erreur sur le module 'command' (Issue :#11)

[v1.1.2]
* Correction d'erreur suite à prise en compte des messages/warnings d'ansible-lint

[v1.1.1]
* Suppression du tag always (Issue: #8)
* Correction d'une erreur sur des versions récentes de ansible (Issue :#9)

[v1.1.0]
* Suppression des configurations par défaut empêchant d'utiliser track_process (Issue: #6)

[v1.0.9]
* Ajout des autorisations SElinux pour les binaires de vérification de la bascule automatique (Fix: #5)

[v1.0.8]
* Gestion de compatibilité avec les version 1.x.x et 2.x.x de la collection `openstack.cloud` 

[v1.0.7]
* Update readme "valeur de keepalived_check_primary_script"

[v1.0.6]
* Possibilité d'utilisé l'état d'un process pour la bascule de VIP

[v1.0.5]
* Fixed: get_vip

[v1.0.4]
* Correction recuperation infos Vip

[v1.0.3]
  * Renommage de la variable 'cloud_name' en 'cloud'
  * Suppression du filtre 'cloud' dans la recherche de la VIP

[v1.0.2]
  * Ajout variable cloud
  * Modification de tasks/get_vip.yml pour ajouter l'étape d'authentification au tenant et le nom du tenant pour la requête sur la vip 

[v1.0.1]
  * Mise a jour du calcul de priorité
  * Récupération dynamique de la VIP à partir du nom fournit en paramètre 

[v1.0.0] Initial release
