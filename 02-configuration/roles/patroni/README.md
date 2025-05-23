# Rôle ansible : patroni

Rôle qui installe et configure Patroni - Solution HA pour PostgreSQL. Il est utilisé avec etcd comme backend.
Pour ajouter un replica à un cluster existant, il faudra surcharger la variable `patroni_add_replica` avec la valeur `true`.

L'exécution de ce rôle a comme prérequis L'exécution du rôle PostGreSQL. Ce rôle a pour but l'initialisation d'un cluster PostgreSQL géré par patroni et a vocation à n'être exécuté qu'une seule fois.


**Attention, la construction du cluster Patroni basé sur la méthode pgbackrest n'est pas supportée.
Pour plus d'informations https://agiledevops-portail.dgfip.finances.rie.gouv.fr/confluence/display/A2C/Pgbackrest+-+Cas+d%27utilisation**

:warning: Ce rôle permet d'exposer nativement des métriques prometheus via le port 8008 sur le réseau d'administration ; il n'est plus nécessaire de déployer patroni-exporter.

:warning: Lors du passage à une v2 de ce rôle, il faut vérifier/modifier le script keepalived_check_cmd du rôle keepalived pour qu'il utilise d'adresse IP du réseau d'admin en lieu et place du réseau data.

# Variables

Les variables suivantes sont à définir (un exemple indicatif est donné) :

```yaml
admin_inet_addr: "{{ admin_ip }}" # Adresse IP de la machine sur le réseau adm ; les réseaux sont alimentés par l'inventaire dynamique
data_inet_addr: "{{ data_ip }}" # Adresse IP de la machine sur le réseau data ; les réseaux sont alimentés par l'inventaire dynamique
etcd_cluster_group_ansible: "platform_etcd" # Nom du groupe ansible des machines hébergeant le cluster etcd ; en cas de colocalisation, peut être `platform_bdd` 
patroni_use_ssl: false # true/false 
postgresql_certs_directory: "" # A définir si patroni_use_ssl est à true   
```

:warning: Si `patroni_use_ssl: true`, il faut positionner sur chaque VM les fichiers suivants, qui sont définis dans les templates jinja du rôle :

```
{{ postgresql_certs_directory}}/server.crt
{{ postgresql_certs_directory}}/server.key
{{ postgresql_certs_directory }}/{{ root_ca_cert_name }}
```


# CHANGELOG

## v2.1.5
- Correctif de la var pour Debian `postgresql_default_service_name` incluant la version, désormais (issue #25)  

## v2.1.4
- Meilleure idempotence du rôle (issue #23)

## v2.1.3
- Suppression des *tags* `always` dans le rôle (Issue #22)

## v2.1.2
- Ajout de paramètres dans le dictionnaire par défaut pour permettre la génération des logs sur Debian (Issue #21)

  - `log_destination: "stderr"`
  - `logging_collector: "true"`
  - `log_filename: "postgresql-%a.log"`
  - `log_rotation_age: "1d"`
  - `log_truncate_on_rotation: "true"`

## v2.1.1
- Ajouts documentaires (Issue #20)

## v2.1.0
- Ajout de la compatibilité Debian (Issue #8)
- Fix Version du package patroni non maitrisée (Issue #19)

## v2.0.2
- Ajouts documentaires (Issue #17)

## v2.0.1
- Ajout d'un warning dans le README

## v2.0.0
- Exposition de l'api rest sur le réseau d'administration. /metrics disponible sur le réseau dradministration
- Suppression de l'utilisateur de rewinder 
- Correction de wal_level à `replica` au lieu de `replicas`.
- Activation du mode failsafe_mode pour l'ETCD
- Déplacement des paramètres statiques postgreSQL vers la section dynamique
- Suppression des tags inutiles.
- Ajout de la possibilité de surcharger la methode de création des réplicas
- Suppression de l'entrée pg_hba dans le fichier patroni.yml, L'utilisation du fichier pg_hba.conf définit dans l'installation PostgreSQL
- Correction [issue #10](https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/role-ansible-dgfip/base-de-donnees/patroni/-/issues/10)
- Ajout de la variable `postgresql_default_config` de type dictionnaire, ajout également de la variable `postgresql_custom_config` afin de surcharger des valeurs par défaut ou d'ajouter de nouvelles propriétés pour le ficher postgresql.conf. Cet ajout rend le role uniquement compatible avec la v2.0.0 du role PostgreSQL. 
- Mise à jour documentaire 

## v1.3.1
- Fix issue #7 : Problème d'ajout de replica avec tablespace 
- Correction d'erreur suite à prise en compte des messages/warnings d'ansible-lint

## v1.3.0
- Possibilité d'ajouter un Replica à un cluster existant.

## v1.2.0
- Variabilisation des paramètres Patoni
- Changement du nom de la variable use_ssl à patroni_use_ssl

## v1.1.5
- Variablisation du chemin du socket postgres
## v1.1.4
- Rendre compatible avec la version 9 de Rocky Linux
## v1.1.3
- Rendre paramétrable le nom du package 'patroni' depuis  la version 8.7 de Rocky Linux
## v1.1.2
- Ajout de variables par défaut: etcd_cluster_group_ansible(string) et etcd_data_directory(list)
- Modification template config.yml.j2 pour ajouter la liste des noeuds etcd via une liste qui peut être surchargée
- Modification template config.yml.j2 dans section postgresql.listen pour écouter sur toutes les ips sinon ne fonctionne pas avec la vip
## v1.1.1
- Changed: the way to choose master host
## v1.2.0 30-11-2022
- Ajout de variables par défaut: etcd_cluster_group_ansible(string) et etcd_data_directory(list)
- Modification template config.yml.j2 pour ajouter la liste des noeuds etcd via une liste qui peut être surchargée
- Modification template config.yml.j2 dans section postgresql.listen pour écouter sur toutes les ips sinon ne fonctionne pas avec la vip
# v1.1.0 19-10-2022
- Activation des fonctionnalités SSL avec la variable 'use_ssl = true/false'.
# v1.0.0 21-09-2022
- Version initiale
