# Role Ansible : grafana

Déploie et configure [grafana](https://github.com/grafana/grafana) - plateforme d'analyse et de surveillance

## Prérequis

- Ansible >= 2.7 (Peut fonctionner sur les versions précédentes, mais sans garantie).
- libselinux-python sur l'hôte de déploiement (uniquement lorsque la machine de déploiement dispose de SELinux)
- grafana >= 5.1 (pour les anciennes versions de grafana, utilisez ce rôle dans la version 0.10.1 ou antérieure).
- jmespath sur l'hôte de déploiement. Si vous utilisez Ansible à partir d'un virtualenv Python, installez *jmespath* sur le même virtualenv via pip.

## Variables

Toutes les variables sont surchargeables et renseignées dans [defaults/main.yml](defaults/main.yml) et dans la table suivante :

| Nom                                | Valeur par défaut                                                                                                                                          | Description                                                                                                                                                                  |
|------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `grafana_use_provisioning`         | true                                                                                                                                                   | Utilisez la fonctionnalité de provisionnement de Grafana lorsque cela est possible (**grafana_version=latest supposera >= 5.0**).                                                |
| `grafana_provisioning_synced`      | false                                                                                                                                                  | S'assure qu'aucun tableau de bord précédemment provisionné n'est conservé s'il n'est plus référencé.                                                                             |
| `grafana_version`                  | 10.1.4                                                                                                                                                 | Version du package Grafana                                                                                                                                                       |
| `grafana_group`                    | grafana                                                                                                                                                | Groupe système Grafana                                                                                                                                                           |
| `grafana_user`                     | grafana                                                                                                                                                | User système Grafana                                                                                                                                                             |
| `grafana_manage_repo`              | true                                                                                                                                                   | Manage package repo (or don't)                                                                                                                                                   |
| `grafana_instance`                 | {{ ansible_fqdn \| default(ansible_host) \| default(inventory_hostname) }}                                                                             | Nom de l'instance Grafana                                                                                                                                                        |
| `grafana_disk`                     | /dev/vdb                                                                                                                                               | Nom de disque à utiliser pour les données grafana                                                                                                                                |
| `grafana_partition`                | /dev/vdb1                                                                                                                                              | Nom de la partition à utiliser pour les données grafana                                                                                                                          |
| `grafana_logs_dir`                 | /var/log/grafana                                                                                                                                       | Chemin d'accès au répertoire des logs                                                                                                                                            |
| `grafana_data_dir`                 | /var/lib/grafana                                                                                                                                       | Chemin d'accès au répertoire de la base de données                                                                                                                               |
| `monitoring_additional_volume`     | false                                                                                                                                                  | Mountage (ou pas) du volume sur la VM                                                                                                                                            |
| `dgfip_download_url`               | <https://nexus3.appli.dgfip/repository/grafana>                                                                                                          | Chemin d'accès au binaire Grafana dans nexus                                                                                                                                     |
| `grafana_address`                  | 0.0.0.0                                                                                                                                                | Adresse d'écoute de grafana                                                                                                                                                      |
| `grafana_port`                     | 3000                                                                                                                                                   | Port d'écoute de grafana                                                                                                                                                         |
| `grafana_cap_net_bind_service`     | false                                                                                                                                                  | Permet l'utilisation de ports inférieurs à 1024 sans privilèges root en exploitant les « capacités » du noyau Linux. Voir: <http://man7.org/linux/man-pages/man7/capabilities.7.html> |
| `grafana_url`                      | "http://{{ grafana_address }}:{{ grafana_port }}"                                                                                                      | URL complète utilisée pour accéder à Grafana depuis un navigateur Web                                                                                                            |
| `grafana_api_url`                  | "{{ grafana_url }}"                                                                                                                                    | URL utilisée pour les appels d'API lors du provisionnement si elle est différente de l'URL publique. Voir [issue #70](https://github.com/cloudalchemy/ansible-grafana/issues/70).|
| `grafana_domain`                   | "{{ ansible_fqdn \| default(ansible_host) \| default('localhost') }}"                                                                                  | Ce paramètre n'est utilisé que dans le cadre de l'option `root_url`. Utile lors de l'utilisation de GitHub ou Google OAuth                                                       |
| `grafana_server`                   | { protocol: http, enforce_domain: false, socket: "", cert_key: "", cert_file: "", enable_gzip: false, static_root_path: public, router_logging: false } | [server](http://docs.grafana.org/installation/configuration/#server) configuration section                                                                                      |
| `grafana_security`  `grafana_security_custom`                 | { admin_user: admin, admin_password: "" }                                                                                                              | [security](http://docs.grafana.org/installation/configuration/#security) configuration section, La variable grafana_security_custom permet de surcharger  certaines propriétés sans redefinir tout le dictionnaire par  défaut (grafana_security).                                                                                |
| `grafana_database`                 | { type: sqlite3 }                                                                                                                                      | [database](http://docs.grafana.org/installation/configuration/#database) configuration section                                                                                   |
| `grafana_welcome_email_on_sign_up` | false                                                                                                                                                  | Envoyer un e-mail de bienvenue après votre inscription                                                                                                                           |
| `grafana_users`                    | { allow_sign_up: false, auto_assign_org_role: Viewer, default_theme: dark }                                                                            | [users](http://docs.grafana.org/installation/configuration/#users) configuration section                                                                                         |
| `grafana_auth`                     | {}                                                                                                                                                     | [authorization](http://docs.grafana.org/installation/configuration/#auth) configuration section                                                                                  |
| `grafana_ldap`                     | {}                                                                                                                                                     | [ldap](http://docs.grafana.org/installation/ldap/) configuration section. group_mappings are expanded, see defaults for example                                                  |
| `grafana_session`                  | {}                                                                                                                                                     | [session](http://docs.grafana.org/installation/configuration/#session) management configuration section                                                                          |
| `grafana_analytics`                | {}                                                                                                                                                     | Google [analytics](http://docs.grafana.org/installation/configuration/#analytics) configuration section                                                                          |
| `grafana_smtp`                     | {}                                                                                                                                                     | [smtp](http://docs.grafana.org/installation/configuration/#smtp) configuration section                                                                                           |
| `grafana_alerting`                 | {}                                                                                                                                                     | [alerting](http://docs.grafana.org/installation/configuration/#alerting) configuration section                                                                                   |
| `grafana_log`                      | {}                                                                                                                                                     | [log](http://docs.grafana.org/installation/configuration/#log) configuration section                                                                                             |
| `grafana_metrics`                  | {}                                                                                                                                                     | [metrics](http://docs.grafana.org/installation/configuration/#metrics) configuration section                                                                                     |
| `grafana_tracing`                  | {}                                                                                                                                                     | [tracing](http://docs.grafana.org/installation/configuration/#tracing) configuration section                                                                                     |
| `grafana_snapshots`                | {}                                                                                                                                                     | [snapshots](http://docs.grafana.org/installation/configuration/#snapshots) configuration section                                                                                 |
| `grafana_image_storage`            | {}                                                                                                                                                     | [image storage](http://docs.grafana.org/installation/configuration/#external-image-storage) configuration section                                                                |
| `grafana_dashboards`               | []                                                                                                                                                     | Liste des dashboards (tableaux de bord) à importer                                                                                                                               |
| `grafana_dashboards_dir`           | "dashboards"                                                                                                                                           | Chemin d'accès à un répertoire local contenant les fichiers de tableaux de bord au format `json`                                                                                 |
| `grafana_datasources`              | []                                                                                                                                                     | Liste des sources de données à configurer                                                                                                                                        |
| `grafana_environment`              | {}                                                                                                                                                     | Paramètre d'environnement facultatif pour l'installation de Grafana, utile par exemple pour définir http_proxy                                                                   |
| `grafana_plugins`                  | grafana-clock-panel, vonage-status-panel, agenty-flowcharting-panel, jdbranham-diagram-panel                                                           | Liste des plugins Grafana qui doivent être installés                                                                                                                             |
| `grafana_alert_notifications`      | []                                                                                                                                                     | Liste des canaux de notification d'alerte à créer, mettre à jour ou supprimer                                                                                                    |
| `plain_pem_tls`                    | false                                                                                                                                                  | Obligatoire si haproxy_tls_enabled = true, cette variable contient le pair clé/certificat (cat tls.crt key.crt), ce paramètre doit être mis dans un fichier encrypté par vault   |
| `grafana_install_rp`               | false                                                                                                                                                  | Installe et configure reverse proxy (haproxy)                                                                                                                                    |
| `grafana_organizations`            | false                                                                                                                                                  | Liste des Organisations à créer. IMPORTANT: l'authentification basique doit être activée                                                                                         |

## Exemple de Datasource

```yaml
grafana_datasources:
  - name: prometheus
    type: prometheus
    access: proxy
    url: 'http://{{ prometheus_web_listen_address }}'
    basicAuth: false
```

## Exemple de Dashboard

```yaml
grafana_dashboards:
  - dashboard_id: 111
    revision_id: 1
    datasource: prometheus
```

## Exemple de notification d'Alerte

**REMARQUE** : la variable `grafana_alert_notifications` ne sera utilisée que lorsque `grafana_use_provisioning` est positionnée à `true`.
Cela signifie que le nouveau système de provisionnement utilisant des fichiers de configuration, disponible à partir de Grafana v5.0, doit être utilisé.

Utiliser un exemple de modèle de dépôt Grafana Yum personnalisé :

- Placez votre template à côté de votre playbook dans le dossier `templates`
- Utilisez un chemin différent de celui par défaut, car lorsque vous utilisez un chemin relatif, ansible utilise le premier modèle trouvé et cherche d'abord sous le répertoire role, puis dans le répertoire playbook.

## Exemple

### Playbook

Remplissez le champ du mot de passe administrateur selon votre choix, la page Web Grafana ne demandera pas de le modifier lors de la première connexion.

```yaml
- hosts: all
  roles:
    - role: dgfip.grafana
      vars:
        grafana_security_custom:
          admin_user: admin
          admin_password: enter_your_secure_password
```

## Changelog

[v1.3.1]

- Fix Issue #17:  Ajout de 'become=false' pour la tache 'delegate_to' avec localhost pour éviter de necessité un mot de passe implicitement sur le controleur.

[v1.3.0]

- Fix Issue #15: Variabilisation de grafana_security.admin_user et grafana_security.admin_password.
- Ajout de la variable grafana_security_custom afin de surcharger des valeurs par défaut ou d'ajouter de nouvelles propriétés sans rédefinir 'grafana_security' dans son ensemble.
- Suppression du répertoire 'molecule'.

[v1.2.4]

- Correctifs mineurs linter, compatibilité Debian.

[v1.2.3]

- Suppression du paramètre déprécié 'warn' sur command sur les versions recentes de ansible-core.

[v1.2.2]

- Mise en conformité Debian
- Ajout du package `acl` pour debian

[v1.2.1]

- Mise en conformité de README

[v1.2.0]

- Ajout de la création d'organisations à l'installation via la varable `grafana_organizations`. Si cette est définie et non-vide, le rôle crée les organisations définies.

Exemple:

```yaml
grafana_organizations:
  - Org_A
  - Org_B
```

[v1.1.0]

- Fonctionnalité d'installation de plugins

[v1.0.6]

- Mise à jour vers la version 9.5.5 de Grafana pour corriger CVE-2023-3128

[v1.0.4]

- Correction Problème de téléchargement

[v1.0.3]

- Correction erreur d'accès

[v1.0.1]

- Utilisation haproxy au lieu de nginx

[v1.0.0]

- Version initiale
