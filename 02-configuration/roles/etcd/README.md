# ansible role etcd

Rôle pour le déploiement d'un cluster etcd (key-value-store) à haute disponibilité sur Rocky Linux 8.x.
Il est utilisé en tant que backend Patroni pour construire un cluster PostgeSQL hautement disponible.

## Variables

Les variables suivantes sont à définir (un exemple indicatif est donné) :

```yaml
admin_inet_addr: "{{ admin_ip }}" # Adresse IP de la machine sur le réseau adm ; les réseaux sont alimenté par l'inventaire dynamique
data_inet_addr: "{{ data_ip }}" # Adresse IP de la machine sur le réseau data ; les réseaux sont alimenté par l'inventaire dynamique
etcd_cluster_group_ansible: "platform_etcd" # Nom du groupe ansible des machines hébergeant le cluster etcd ; en cas de colocalisation, peut être `platform_bdd` 
etcd_cluster_name: "etcd-cluster-01" # Nom du cluster etcd
etcd_initial_cluster_state: "new" # 'new' pour l'initialisation du cluster, 'existing' si le noeud est reconstruit. 
etcd_volume_mntp: "/etcdata" # base de l'arborescence des données etcd
etcd_use_ssl: false # true/false 
force_install: false # true/false 
certs_path: "" # A définir si etcd_use_ssl est à true   
```endpoint

:warning: Si `etcd_use_ssl: true`, il faut positionner sur chaque VM les fichiers suivants, qui sont définis dans les templates jinja du rôle :

- "{{ certs_path }}/{{ ansible_hostname }}.pem"
- "{{ certs_path }}/{{ ansible_hostname }}.key"
- "{{ certs_path }}/{{ root_ca_cert_name }}"

Gestion du niveau de logs :

```yaml
etcd_log_level: "error" # par défaut info ; les valeurs acceptées sont debug, info, warn, error, panic ou fatal
```

## CHANGELOG 

### v2.0.4 09-12-2024

- Correction ansible-lint (Fix issue #11)

### v2.0.3 18-09-2024

- Ajout de controle pour installer un noeud  que lorqu'une installation n'est pas déjà faite.
- Ajout de la variable `etcd_initial_cluster_state` pour permettre la reconstruction d'un noeud. Par défaut `new`, en cas de reconstruction `existing`
- Ajout de la variable `force_install` pour permettre de réinstaller le noeud  même si une installation est déjà faite.Par défaut `false`

### v2.0.2 10-09-2024

- Ajout de la variable `etcd_log_level` pour gérer le niveau de log (par défaut, `info` ; Issue #9)

### v2.0.1 23-08-2024

- Ajout de la compatibilité debian (Issue #4)
- Ajouts documentaires (Issue #8)
- Modification de la variable `use_ssl` en `etcd_use_ssl`

### v2.0.0 22-05-2024

- Montée de version de l'api v2 vers la v3

### v1.2.0 20-02-2024

- Ajout de variables par défaut
- Modification template etcd.conf.j2 pour que la liste des membres soit plus dynamique et que le nom du groupe ansible ne soit pas figé
- Exposition sur le réseau admin pour la collecte des metrics

### v1.1.0 19-10-2022

- Activation des fonctionnalités SSL avec la variable 'use_ssl = true/false'.

### v1.0.0 21-09-2022

- Version initiale
