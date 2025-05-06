# Ansible Role: PostgreSQL

Ce rôle permet d'installer et de configurer PostgreSQL officiellement en version  14 (version installée par défaut) et 16. sur les serveurs Rocky Linux 8 et 9 et Debian 12.

## Requirements

Les données doivent idéalement être stockées sur un volume dédié. Le rôle <https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/role-ansible-dgfip/utilitaires/manage-lvm> vous permettra de configurer un volume et le gérer avec LVM.

Notez juste que ce rôle nécessite un accès root.

## Variables

Les variables disponibles sont énumérées ci-dessous, ainsi que les valeurs par défaut. (voir `defaults/main.yml`):

Propriétés du volume pour le stockage des données: (Si la variable filesystem_list n'est pas définie, le rôle 'manage-lvm' récupère automatiquement les meta données positionnées au moment de la création du volume pour le configurer.)

~~~~
Pour rappel, une étude Dalibo nous a permis de proposer une configuration par défaut optimale.
Néanmoins vous êtes libre d'apporter des modifications / ajouts par rapport à votre contexte projet. 
~~~~

```yaml
    filesystem_list:
      - {
          lv_name: "postgresql_data",
          mntp: "/pgdata",
          device: "/dev/vdb",
          vg_name: "postgresql_data_Vg",
          owner: "root",
          group: "root",
          mode: "0755"
        }
```

Configuration par défaut de l'instance Postgresql:

```yaml
    postgresql_default_config:
      wal_level: "replica"
      hot_standby: "on"
      max_connections: "100"
      max_wal_senders: "10"
      max_replication_slots: "5"
      max_prepared_transactions: "0"
      max_locks_per_transaction: "64"
      wal_log_hints: "off"
      track_commit_timestamp: "off"
      log_rotation_size: "100MB"
      port: "5432"
      listen_addresses: "172.14.0.52"
      ...
```

Il est possible de sucharger ou d'en rajouter de nouvelles propriétés sans redéfinir le dictionnaire dans son ensemble. Pour cela il faut définir `postgresql_custom_config` :

```yaml
    postgresql_custom_config:
      max_connections: "200"
      port: "54320"
      listen_addresses: "{{ hostvars[inventory_hostname]['ansible_default_ipv4']['address'] }}"
```

De la même manière, les dictionnaires `postgresql_default_sysctl_config` et `postgresql_custom_sysctl_config` vont permettre de définir les coniguration 'sysctl' par défaut et de pouvoir les surcharger ou d'en rajouter si besoin:

```yaml
    postgresql_default_sysctl_config:
      vm.overcommit_memory: "2"
      vm.swappiness: "10"
      ...
```

```yaml
    postgresql_custom_sysctl_config:
      vm.swappiness: "30"
      vm.overcommit_ratio: "80"
```

Il est recommandé pour une base de données postgresql d'activer la swap sur la VM.
Le paramètre `postgresql_vm_swap_file_create` permet de créer/activier une swap sur la VM en fonction des paramètres `postgresql_vm_swap_file_path` et `postgresql_vm_swap_file_size_mb`, si la valeur est à true. Par défaut la valeur est à 'false'. 
A2C recommande fortement d'utiliser le rôle `manage-lvm` qui prend désormais la création et l'activation de SWAP sur la VM. Néamoins les variables suivantes sont disponibles au cas où:

```yaml
    postgresql_vm_swap_file_create: false  # par défaut
    postgresql_vm_swap_file_path: /swapfile
    postgresql_vm_swap_file_size_mb: "2048"
```

Propriétés de la base de données à créer: ( la propriété )

```yaml
    postgresql_db_name: "a2c_pg"
    postgresql_db_username: "a2c_pg_user"
    postgresql_root_password: "{{ vault_postgresql_root_password }}"
    postgresql_db_password: "{{ vault_postgresql_db_password }}"
    postgresql_data_directory: "/pgdata/{{ postgresql_db_name }}/data"
    postgresql_login_host: localhost
```

Le paramètre `postgresql_wal_directory` est déduit à partir de `postgresql_data_directory` et nom surchargeable par l'utilisateur.

```yaml
postgresql_wal_directory: "{{ postgresql_data_directory | dirname }}/pg_wal"
```

L'utilisateur et le groupe sous lesquels PostgreSQL sera exécuté:

```yaml
    postgresql_user: postgres
    postgresql_group: postgres
```

Liste des bases de données à créer sur le serveur. Seul le paramètre `name` est obligatoire ; toutes les autres propriétés sont facultatives.

```yaml
    postgresql_databases:
      - name: exampledb # required; the rest are optional
        lc_collate: # defaults to 'fr_FR.UTF-8'
        lc_ctype: # defaults to 'fr_FR.UTF-8'
        encoding: # defaults to 'UTF-8'
        template: # defaults to 'template0'
        login_host: # defaults to {{ postgresql_login_host }}
        login_password: "{{ postgresql_root_password }}"
        login_user: # defaults to 'postgresql_user'
        login_unix_socket: # defaults to 1st of postgresql_unix_socket_directories
        port: # defaults to not set
        owner: # defaults to postgresql_user
        state: # defaults to 'present'
        schemas: 
          - 'schema_1'
          - 'schema_2'
```

Liste des utilisateurs à créer sur le serveur. Seul le paramètre `name` est obligatoire ; toutes les autres propriétés sont facultatives.

```yaml
    postgresql_users:
      - name: jdoe #required; the rest are optional
        password: # defaults to not set
        encrypted: # defaults to not set
        priv: ALL
        role_attr_flags: # defaults to not set
        db: # defaults to not set
        login_host: # defaults to {{ postgresql_login_host }} 
        login_password: "{{ postgresql_root_password }}"
        login_user: # defaults to '{{ postgresql_user }}'
        login_unix_socket: # defaults to 1st of postgresql_unix_socket_directories
        port: # defaults to not set
        state: # defaults to 'present'
```

Divers:

   `postgresql_disable_gpg_check` : si la valeur est à true, la vérification GPG sera ignorée pendant l'installation des paquets rpm via yum.

## Example Playbook

```yaml
    - hosts: database
      become: yes
      vars_files:
        - vars/main.yml
      roles:
        - postgresql
```

*contenu de vars/main.yml*:

```yaml
  postgresql_use_ssl: false
  postgresql_db_name: "a2c_pg"
  postgresql_db_username: "a2c_pg_user"
  postgresql_db_password: "{{ vault_postgresql_db_password }}"
  postgresql_data_directory: "/pgdata/{{ postgresql_db_name }}/data"

  filesystem_list:
    - {
      lv_name: "postgresql_data",
      mntp: "/pgdata ",
      size: "5g",
      device: "/dev/vdb",
      vg_name: "postgresql_data_Vg",
      owner: "root",
      group: "root",
      mode: "0755"
    }

  postgresql_databases:
    - name: "{{ postgresql_db_name }}"
  postgresql_users:
    - name: "{{ postgresql_db_username }}"
      password: "{{ postgresql_db_password }}"

  postgresql_default_config:
    wal_level: "replica"
    hot_standby: "on"
    max_connections: "100"
    max_wal_senders: "10"
    max_replication_slots: "5"
    max_prepared_transactions: "0"
    max_locks_per_transaction: "64"
    wal_log_hints: "off"
    track_commit_timestamp: "off"
    log_rotation_size: "100MB"
    port: "5432"
    listen_addresses: "172.14.0.52"
    data_directory: "{{ postgresql_data_directory }}"
    hba_file: "{{ postgresql_conf_directory }}/pg_hba.conf"
    ident_file: "{{ postgresql_conf_directory }}/pg_ident.conf"
    external_pid_file: "/var/run/postgresql/pgsql.{{ postgresql_version }}.pid"
    unix_socket_directories: "/var/run/postgresql"
    log_directory: "/var/log/postgresql"
    ssl: "{{ (postgresql_use_ssl == true) | ternary('on', 'off') }}"
    ssl_ca_file: "{{ postgresql_certs_directory }}/root-ca.pem"
    ssl_cert_file: "{{ postgresql_certs_directory }}/server.crt"
    ssl_key_file: "{{ postgresql_certs_directory }}/server.key"
    autovacuum_max_workers: "3"
    huge_pages: "try"
    shared_buffers: "{{ ((ansible_memtotal_mb / 4) | round | int) }}MB"
    temp_file_limit: "10GB"
    maintenance_work_mem: "{{ ((ansible_memtotal_mb * 0.25) | round | int) }}MB"
    min_wal_size: "512MB"
    max_wal_size: "2GB"
    max_worker_processes: "{{ ansible_processor_nproc }}"
    max_parallel_workers: "{{ ansible_processor_nproc }}"
    effective_cache_size: "{{ (((ansible_memtotal_mb / 1000) * (2 / 3)) | round | int) }}GB"
    random_page_cost: "4"
    checkpoint_flush_after: "256kB"
```

## HISTORIQUE DES VERSIONS (CHANGELOG)

## v2.1.7  12-03-2025
- Ajout règle SELinux sur Debian pour les propriétés `allow_execmem` et `allow_execstack`.

## v2.1.6  05-02-2025
- Correctif de gestion des "databases", si le dictionnaire fourni ne contient pas de champ `login_password` ; par défaut `postgresql_root_password` (issue #65)
- Amélioration de l'idempotence sur l'étape SELinux sur pg_ctl (issue #66)

## v2.1.5  28-01-2025
- Limitation des informations affichées dans la log (issue #64)
- Ajout de `log_line_prefix` dans `postgresql_default_config` (*reopen* issue #46)
- Ajout règle SELinux sur le contexte des logs

## v2.1.4  10-12-2024
- Ajout du démarrage au boot du service `postgresql@{{ postgresql_version }}-main` en environnement debian (issue #62)

### v2.1.3 28-10-2024
- AJout d'un contexte SELinux supplémentaire (Issue #56)
- Correctif SELinux sur la gestion des tablespaces (Issue #57)
- Suppression du tag `always` de certaines tâches de set_fact (Issue #58)

### v2.1.2 08-10-2024

- Ajout de paramètres dans le dictionnaire par défaut pour permettre la génération des logs sur Debian (Issue #52)

  - `log_destination: "stderr"`
  - `logging_collector: "true"`
  - `log_filename: "postgresql-%a.log"`
  - `log_rotation_age: "1d"`
  - `log_truncate_on_rotation: "true"`

- Correction du contexte SELinux du binaire postgres (Issue #45)

### v2.1.1 26-09-2024
- Remise en *defaults* de la variable `postgresql_service_name` (issue #50)

### v2.1.0 10-09-2024
- Ajout de la compatibilité Debian (Issue #32)

### v2.0.6 03-09-2024

- Fix issue #47 : Problème sur la tâche 'Check if PostgreSQL port is open' lorsque plusieurs adresses sont contenues dans la variable 'postgresql_config.listen_addresses'

### v2.0.5 30-07-2024

- Mise à jour de la config pour garder les valeurs par défaut des propriétés suivante :

  - bgwriter_flush_after
  - backend_flush_after
  - wal_writer_flush_after

### v2.0.4 08-07-2024

- Retour arrière sur "Suppresion de `Param 'priv' is deprecated` pour le paramètre `postgresql_users`"

### v2.0.3 25-06-2024

- En cas de rejeu, redemarrage de l'instance postgresql seulement si un élement de configuration à changé.
- Fix issue #42:  Erreur avec la commande initdb et la variable postgresql_wal_directory. Ne pas permettre une redéfinition du paramètre par l'utilisateur.
- Suppresion de `Param 'priv' is deprecated` pour le paramètre `postgresql_users`

### v2.0.2 30-05-2024

- Permettre le rejeu en skippant les étapes si patroni est installé

### v2.0.1 30-05-2024

- Fix Issue #40 : Retour arrirère sur la recommandation Dalibo pour le scheduler disque.

### v2.0.0 25-05-2024

- Application des recommandations DALIBO : configuration postgresql et les optimisations 'system' (sysctl) de la VM.
- La variable `postgresql_global_config_options` est  rénommée en `postgresql_default_config` de type dictionnaire.
- Ajout de la variable `postgresql_custom_config` afin de surcharger des valeurs par défaut ou d'ajouter de nouvelles propriétés pour le ficher postgresql.conf.
- Ajout la variable `postgresql_default_sysctl_config` pour les optimisation 'system' par défaut de type dictionnaire.
- Ajout de la variable `postgresql_custom_sysctl_config` afin de surcharger des valeurs par défaut ou d'ajouter de nouvelle propriétés pour la partie système.
- Fix Issue #37 : La vérification de la définition des variables semble ne pas fonctionner.

### v1.4.5 08-04-2024

- Ajout de l’interruption de l'installation pour les instances gérées par Patroni

### v1.4.4 28-03-2024

- Valider que le répertoire contenant les TABBLESPACEs n'est pas un sous-répertoire de PGDATA en cas de création de tablespace (Issue #31).

### v1.4.3 19-03-2024

- Correctifs linter ansible (Issue #29)
- Meilleure gestion du port postgreqsql (Issue #30 "Erreur si utilisation d'un port exotique")

### v1.4.2 07-02-2024

- Fix issue #26 : Possibilité de créer plusieurs schémas avec 1 ou plusieurs base de données sur la même VM.

### v1.4.1 01-02-2024

- Fix issue #24 : Regex postgresql.conf trop permissive

### v1.4.0 05-01-2024

- Possibilité de créer une base de donnée avec plusieurs schémas
- Permettre d'autoriser le contournement des erreurs de vérification GPG sur les paquets yum

### v1.3.0 21-11-2023

- Variabilisation de la config postgres pour Patroni

### v1.2.8 30-10-2023

- Variabilisation du port d'écoute pour les tests
- Variabilisation du host d'écoute pour les tests
- Validation de l'installation de la version 16 de postgresql

### v1.2.7 24-10-2023

- Ajout de test pour démmarage du service

### v1.2.6 13-10-2023

- Fix de la version v1.2.3 : faire l'association entre database et tablespace
  On peut désormais créer et associer un tablespace à une base de la manière suivante pour la définition de la variable  'postgresql_databases':
  
  postgresql_databases:
  - name: "test"
    login_password: "test"
    tablespace: "test_tbs"
  
  puis définir la variable 'postgresql_tablespaces' de la manière suivante:

  postgresql_tablespaces:
  - name: "test_tbs"
    location: "/data/test_tbs"

### v1.2.5 26-09-2023

- Evolution du rôle afin de pouvoir faire l'association entre database et tablespace.

### v1.2.4 23-03-2023

- Ajout de l'authentification via localhost lorsque IPv6 est activé.
- Mise à jour pour la compatibilité Rocky 9.

### v1.2.3 23-03-2023

- Révocations sur le schéma public: ajout de test d'existence de ce schéma.

### v1.2.2 02-03-2023

- Ajout de l'option fail_on_error pour le rôle postgres (module postgresql_privs).
- Permettre de garder à jour les versions déployées.
- Supression de la variable.

### v1.2.1 01-02-2023

- Ajout de la possibilité d'utiliser partout une variable postgresql_login_host (dont la valeur par défaut est défaut est localhost).

### v1.2.0 30-01-2023

- Fixed : La variable use_ssl n'est pas homogène avec les autres variables du rôle.
- Ajout de la possibilité de faire des queries en post-installation.

### v1.1.4 20-12-2022

- Rendre la propriété 'login_host' paraméable afin d'utiliser l'addresse IP souhaitée autre que 'localhost' par déut. La valeur par défaut reste 'localhost'.

### v1.1.3 15-12-2022

- Correction de l' éluation de la condition pour l'utilisation du SSL.

### v1.1.2 05-12-2022

Rendre paraméable le mode par déut de 'selinux'.

### v1.1.1 30-11-2022

- Suppression du template postgresql.conf.j2.
- Remplacement de la task template pour postgresql.conf.j2 par une task lineinfile pour remplacer dans le fichier postgresql.conf par déut par chaque option:value contenu dans le dictionnaire postgresql_global_config_options.

### v1.1.0 19-10-2022

- Activation des fonctionnalités SSL avec la variable 'use_ssl = true/false'.
- Suppression des droit par défaut qui autorise à créer un objet dans le schéma public de la BD créée pour tous les utilisateurs.

### v1.0.0 29-07-2022

- Version initiale
