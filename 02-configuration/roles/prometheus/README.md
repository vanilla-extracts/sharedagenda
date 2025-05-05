# Role Ansible : prometheus

## Description

Déployer le système de surveillance Prometheus à l'aide d'ansible.

## Variables

Toutes les variables qui peuvent être modifiées sont stockées dans le fichier [defaults/main.yml](defaults/main.yml) ainsi que dans le tableau ci-dessous.

| Name                               | Valeur par défaut                                   | Description                                                                |
|------------------------------------|-------------------------------------------------|----------------------------------------------------------------------------|
| `prometheus_version`               | 2.45.1                                          | Version du paquet Prometheus. Seul Prometheus 2.x est pris en charge.               |
| `prometheus_skip_install`          | false                                           | Les tâches d'installation de Prometheus sont ignorées lorsqu'elles sont réglées sur true (vrai)                |
| `monitoring_additional_volume`     | false                                           | Paramètre booléen, pour monter ou non un volume sur {{ répertoire_de_surveillance }}        |
| `prometheus_config_dir`            | /etc/prometheus                                 | Chemin d'accès au répertoire contenant la configuration de prometheus                            |
| `monitoring_dir`                   | /var/lib/monitoring                             | Chemin d'accès au répertoire de surveillance                                               |
| `prometheus_db_dir`                | {{ monitoring_dir }}/prometheus                 | Chemin d'accès au répertoire contenant la base de données prometheus                                 |
| `prometheus_system_group`          | prometheus                                      | Groupe Prometheus System                                                    |
| `prometheus_system_user`           | prometheus                                      | Utilisateur du système Prometheus                                                     |
| `prometheus_web_listen_address`    | 0.0.0.0                                         | Adresse sur laquelle prometheus sera à l'écoute                             |
| `prometheus_web_listen_port`       | 9090                                            | Port sur lequel prometheus écoutera                                |
| `dgfip_download_url`               | <https://nexus3.appli.dgfip/repository/prometheus>| URL du binaire loki dans Nexus Cloud                                          |
| `prometheus_web_external_url`      | /prometheus                                     | Adresse externe sur laquelle prometheus est disponible                          |  
| `prometheus_storage_retention`     | 30d                                             | Durée de conservation des données par taille                                              |  
| `prometheus_storage_retention_size`| 0                                               | Drapeaux de configuration supplémentaires transmis au binaire prometheus au démarrage      |  
| `prometheus_config_flags_extra`    | {}                                              | Port sur lequel prometheus écoutera                                 |  
| `prometheus_alertmanager_config`   | []                                              | Configuration chargée d'indiquer où se trouvent les gestionnaires d'alerte            |  
| `prometheus_alert_relabel_configs` | []                                              | Règles de réétiquetage des alertes. Elles doivent être spécifiées sous la forme d'une liste au format yaml.    |  
| `prometheus_global`                | { scrape_interval: 60s, scrape_timeout: 15s, ..}| Configuration globale de Prometheus. Compatible avec la configuration officielle           |  
| `prometheus_remote_write`          | []                                              | Écriture à distance. Compatible avec la configurations  officielle                        |  
| `prometheus_remote_read`           | []                                              | Lécture à distance. Compatible avec la configuration officiele                        |  
| `prometheus_external_labels`       | environment: "{{ ansible_fqdn }}"               | default(ansible_host)                                                      |
| `prometheus_targets`               | {}                                              | Cibles qui seront éliminées.                                             |
| `prometheus_scrape_configs`        | see [defaults/main.yml](defaults/main.yml)      | Les travaux de recherche de Prometheus sont fournis dans le même format que dans les documents officiels.         |
| `prometheus_config_file`           | template/prometheus.yml.j2                      | Variable utilisée pour fournir un modèle de fichier de configuration prometheus personnalisé     |
| `prometheus_group_alert_rules`     | [defaults/main.yml](defaults/main.yml)          | Le nom du groupe de règles d'alerte |  
| `prometheus_alert_rules`           | [defaults/main.yml](defaults/main.yml)          | Liste complète des règles d'alerte qui seront copiées (ansible_managed.rules)  |  
| `prometheus_alert_rules_files`     | see [defaults/main.yml](defaults/main.yml)      | Liste des dossiers ou fichiers contenant des règles d'alerte qui seront copiés    |
| `prometheus_static_targets_files`  | see [defaults/main.yml](defaults/main.yml)      | Liste des dossiers ou fichiers contenant la configuration statique personnalisée de la cible     |
| `prometheus_file_sd_configs_format`| yaml                                            | Définir le format de configuration de file_sd (yaml ou json) |

## Backup Variables

| Name                           | Default Value                                        | Description                                               |
|--------------------------------|------------------------------------------------------|-----------------------------------------------------------|
| `prometheus_backup_enabled`        | false                             | Par défaut on skip la partie sauvegardes            |
| `component_name`        | "prometheus"                             | Nom du composant             |
| `prometheus_backup_config`                | Voir exemple ci-dessous.                               | Dictionnaire permettant de définir les propriété de sauvegarde. |
| `restic_command_opts`        | ""                                                   | Permet de rajouter des options à la commande restic               |
| `restic_common_utils`        | "/usr/local/bin/restic-utils.sh"                  | Fichier des fonctions communes pour la sauverage. Voir <https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/role-ansible-dgfip/utilitaires/restic-core/-/blob/main/templates/utils.sh.j2?ref_type=heads> pour les détails.                      |
| `prometheus_backup_folder`         | "{{ monitoring_dir }}/backup/{{ component_name }}" | Emplacement des fichiers pour la sauvegarde locale.                  |
| `prometheus_backup_scripts`        | "{{ monitoring_dir }}/scripts/{{ component_name }}"| Emplacement des scripts pour la sauvegarde                          |
| `prometheus_restore_folder`        | "{{ monitoring_dir }}/restore/{{ component_name }}"| Emplacement des données restorées   |
| `prometheus_data_folder`           | "{ {monitoring_dir }}/{{ component_name }}"    | Emplacement des données à sauvegader                                     |
| `prometheus_backup_keep_days`      | 30 | Permet de supprimer les sauvegardes (fichiers ou dossiers) datant de plus de prometheus_backup_keep_days jours |
| `prometheus_backup_keep_locally`   | 30 | Permet de concerver les sauvegardes (fichiers ou dossiers) datant de plus de prometheus_backup_keep_locally jours   |
| `prometheus_restic_logdir`         | "{{ monitoring_dir }}/logs/{{ component_name }}"               | Répertoire des logs  |
| `prometheus_restic_password`       | "prometheus_backup"                    | Le mot de passe restic, à ne pas perdre, peut être défini via la variable  |
| `prometheus_restic_pack_size`      | "16MB"                     |          Taille du pack size, optimisation possible lors du backup  |
| `prometheus_os_application_credential_id` | ""          | ID de l'identifiant d'application Swift du projet |
| `prometheus_os_application_credential_secret` | "" | Secret de l'identifiant d'application Swift  du projet, Exp: <https://nuage01.dgfip.finances.rie.gouv.fr:5000/v3>  |
| `prometheus_os_auth_url` | "true"                                   | Url pour l'authentification pour la platforme qui héberge le projet          |
| `prometheus_os_swift_bucket` | "{{ component_name }}"  | Nom du bucket swift au sein du projet  |
| `prometheus_os_swift_folder` | "backup" | Nom du répertoire dans le bucket swift |

Exemple de définition pour `prometheus_backup_config` :

```yaml
prometheus_backup_enabled: true
prometheus_backup_config:
  prune: true
  list_files: false
  compress: true
  recycle: true
  list_snapshots: true
  cron:
    weekday: 1-5
    minute: "0~59"
    hour: 23
```

### Relation entre `prometheus_scrape_configs` et `prometheus_targets`

#### Version courte

`prometheus_targets` est juste une carte utilisée pour créer plusieurs fichiers situés dans le répertoire "{{ prometheus_config_dir }}/file_sd". Les noms des fichiers sont composés à partir des clés de premier niveau de cette carte avec le suffixe `.yml` ou `.json`. Ces fichiers stockent [file_sd scrape targets data] (<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#file_sd_config>) et doivent être lus dans `prometheus_scrape_configs`.

#### Version longue

Une partie du fichier de configuration *prometheus.yml* qui décrit ce qui est scrappé par prometheus est stockée dans `prometheus_scrape_configs`. Pour cette variable, les mêmes options de configuration que celles décrites dans [prometheus docs] (<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#><scrape_config>) sont utilisées.

Quant à `prometheus_targets`, c'est notre façon d'adopter [prometheus scrape type `file_sd`] (<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#><file_sd_config>). Il définit une carte des fichiers avec leur contenu. Les clés de premier niveau sont les noms de base des fichiers qui doivent avoir leur propre tâche de scrape dans `prometheus_scrape_configs` et les valeurs sont le contenu de ces fichiers.

Tout ceci signifie que vous pouvez utiliser des `prometheus_scrape_configs` personnalisés avec `prometheus_targets` fixé à `{}`. Cependant, lorsque vous définissez quoi que ce soit dans `prometheus_targets`, cela doit être mappé à `prometheus_scrape_configs`. Si ce n'est pas le cas, vous obtiendrez une erreur lors des vérifications avant le vol.

#### Example

Regardons notre configuration par défaut, qui montre toutes les fonctionnalités. Par défaut, nous avons ce `prometheus_targets` :

```yaml
prometheus_targets:
  node:  # This is a base file name. File is located in "{{ prometheus_config_dir }}/file_sd/<<BASENAME>>.yml"
    - targets:              #
        - localhost:9100    # All this is a targets section in file_sd format
      labels:               #
        env: test           #
        job: node-exporter  #
```

Cette configuration aura pour effet de créer un fichier nommé `node.yml` dans le répertoire `{{ prometheus_config_dir }}/file_sd`.

Ensuite, ce fichier doit être chargé dans la configuration de scrape. Voici une version modifiée de notre configuration par défaut `prometheus_scrape_configs` :

```yaml
prometheus_scrape_configs:
  - job_name: "prometheus"    # Custom scrape job, here using `static_config`
    metrics_path: "/metrics"
    static_configs:
      - targets:
          - "localhost:9090"
  - job_name: "example-node-file-servicediscovery"
    file_sd_configs:
      - files:
          - "{{ prometheus_config_dir }}/file_sd/node.yml" # This line loads file created from `prometheus_targets`
```

Autre exemple avec le format Json:

```yaml
prometheus_scrape_configs:
  - job_name: "prometheus"    # Custom scrape job, here using `static_config`
    metrics_path: "/metrics"
    static_configs:
      - targets:
          - "localhost:9090"
  - job_name: "example-node-file-servicediscovery"
    file_sd_configs:
      - files:
          - "{{ prometheus_config_dir }}/file_sd/node.json" # This line loads file created from `prometheus_targets`
```

**Note:** pour générer des fichiers cibles au format json, définissez ce paramètre : `prometheus_file_sd_configs_format : "json"`

### Exemple Playbook

```yaml
---
- hosts: all
  roles:
  - dgfip.prometheus
  vars:
    prometheus_targets:
      node:
      - targets:
          - localhost:9100
        labels:
          env: test
          job: node-exporter
    prometheus_scrape_configs:
      - job_name: node-exporter
        file_sd_configs:
        - files:
          - "{{ prometheus_config_dir }}/file_sd/node.json" # This line loads file created from `prometheus_targets`
```

### Playbook - cas complexe

```yaml
---
- hosts: monitoring
  become: yes
  vars:
    prometheus_targets:
      node: []
      mysql: []
      haproxy: []  
      alertmanager:
        - localhost:9093  
    default_node_targets:
      - localhost:9100
    default_haproxy_targets: []
    default_mysql_targets: []
    project_files_path: "./03_config_os_middleware/files"
  tasks:
    - name: "reset variable prometheus_alert_rules_files"
      set_fact:
        prometheus_alert_rules_files: []
    
    - name: "add default rules to variable prometheus_alert_rules_files"
      set_fact:
        prometheus_alert_rules_files: "{{ prometheus_alert_rules_files  + [item] }}"
      loop:
        - prometheus/rules/*.rules
        - "{{project_files_path}}/rules/hardware-hosts.rules"
        - "{{project_files_path}}/rules/prometheus-self-monitoring.rules"

     ## create dynamic variable prometheus_targets for role prometheus  
    - name: "set prometheus_targets => add all node"
      set_fact:
        prometheus_targets: "{{ prometheus_targets | combine ({ item.type : {'targets': default_node_targets | union( item.group | zip_longest([], fillvalue=item.sufixe) | map('join') | list ) }}) }}"
      loop: 
        - {type: 'node', group: "{{ groups.all | difference(groups.bastion) }}", sufixe: '_adm:9100' }
        - {type: 'haproxy', group: "{{ groups.haproxy }}", sufixe: '_adm:9101' }
        - {type: 'mysql', group: "{{ groups.sql_servers }}", sufixe: '_adm:9104' }

    - name: "add items to variable prometheus_alert_rules_files"
      set_fact:
        prometheus_alert_rules_files: "{{ prometheus_alert_rules_files + [project_files_path + '/rules/'+ item.key +'.rules'] }}"
      with_dict: "{{ prometheus_targets }}"
      when: item.key in ['haproxy','mysql','postgresql'] and item.value != []

    - name: install prometheus
      import_role:
        name: prometheus
      vars:
        prometheus_version: 2.38.0
        prometheus_config_flags_extra: 
          enable-feature: remote-write-receiver
        prometheus_scrape_configs:
          - job_name: "prometheus"
            metrics_path: "{{ prometheus_metrics_path }}"
            static_configs:
            - targets:
                - "localhost:9090"
          - job_name: "node"
            file_sd_configs:
            - files:
              - "{{ prometheus_config_dir }}/file_sd/node.yml"
          - job_name: "mysql"
            file_sd_configs:
            - files:
              - "{{ prometheus_config_dir }}/file_sd/mysql.yml"
          - job_name: "haproxy"
            file_sd_configs:
            - files:
              - "{{ prometheus_config_dir }}/file_sd/haproxy.yml"
          - job_name: "alertmanager"
            file_sd_configs:
            - files:
              - "{{ prometheus_config_dir }}/file_sd/alertmanager.yml"
        prometheus_alertmanager_config:
          - scheme: http
            static_configs:
              - targets:
                - localhost:9093
```

**Remarque:** il est recommandé de stocker les variables dans les fichiers group_vars et host_vars pour alléger le playbook.

## Changelog

[v3.0.0]

- Intégration des backup restic dans le rôle. (issue #22)

[v2.1.9]

- Modification droits sous-directory rules de 0640 à 0750 (issue #24)

[v2.1.8]

- Suppression optionnelle des alertings file par la directive `force_purge_rules` (issue #20)

[v2.1.7]

- Prise en charge du socle Debian 12.

[v2.1.6]

- Prise en compte du restart du service si changement de la variable `prometheus_web_external_url` avant les tests [ Issue #16 ]
- Amélioration des tests de vie de prometheus [ Issue #17 ]

[v2.1.5]

- Mise en conformité du readme
- Bug de validation de la configuration en mode json [ Issue #15 ]

[v2.1.4]

- Bug de download de l'archive / merge du 2022-10-13 [ Issue #13 ]

[v2.1.3]

- Eviter une réinstallation en cas de rejeux meme si la même version est déjà présente [issue #12]
- Suppression des 'warnings/erreur' ansible-lint

[v2.1.2]

- Corrections:
  - Ajout d'un "recurse: true" pour être sûr que les fichiers présents dans le répertoire data aient les bons droits [issue #11]
  - Ajout à nouveau du "cas complexe" mais légèrement modifié pour faire fonctionner la génération des fichiers targets au format json [issue #10]

[v2.1.1]

- Suppression de l'exemple "cas complexe" qui n'est plus d'actualité et adaptation de la doc [issue #10]

[v2.1.0]

- Possibilité de générer les fichiers targets au format json (en plus du yaml) [issue #9]

[v2.0.1]

- Variabilisation du nom du group de règles

[v2.0.0]

- Ajout de test de vie après l'installation
- Suppression de l'installation de haproxy (Si besoin de haproxy, l'install doit se faire via le rôle [haproxy](https://forge.dgfip.finances.rie.gouv.fr/dgfip/si1/dan-a2c/role-ansible-dgfip/r-seau/haproxy)
- Changement de l'URL de téléchargement par défaut (nexus-cloud --> nexus3)
- Suppression des fichiers vars/redhat.yml et vars/rocky.yml pour les merger dans vars/main.yml

[v1.4.0]

- Correction du template jinja pour définir les targets via fichier dans file_sd
- Mise à jour version prometheus par defaut

[v1.3.0]

- Possibilité d'activation  de l'API admin (prometheus_admin_api_enabled, activé par défaut)
- Ajout de l'authentification basique pour accéder à prometheus (prometheus_basic_auth_users_enabled = false par défaut) **Cette évolution rajoute une dépendance de module python 'passlib' (pip install passlib ou le rajouter directement dans requirment.txt) pour hashé le mot depasse de l'admin selon l'algorithme 'bcrypt'.**

[v1.2.0]

- Correction du template jinja pour définir les targets via fichier dans file_sd car manque le "- targets" en début de fichier
- Ajout de fichiers de rules pour node exporter, haproxy (optionnel), mysql (optionnel) et postgres (optionnel) à ajouter dans la config Prometheus
- Update README and defaults/main.yml

[v1.1.0]

- add tls capabilities
- replace nginx with haproxy
- Update README and defaults/main.yml

[v1.0.0]
