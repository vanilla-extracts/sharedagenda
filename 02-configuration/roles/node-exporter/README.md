## Description

Déployer le node exporter prometheus en utilisant ansible.

## Exigences
- Ansible >= 2.5 (Il est possible que cela fonctionne sur des versions antérieures, mais nous ne pouvons pas le garantir)
- Pour utiliser ce rôle, vous devez ajouter la collection community.general à votre playbook
```yaml
  - name: community.general
    source: https://nexus-cloud.appli.dgfip/repository/ansible_galaxy/
    version: 6.0.1 
```
## Variables de rôle

Toutes les variables qui peuvent être modifiées sont stockées dans [defaults/main.yml](defaults/main.yml) et sont répertoriées dans le tableau ci-dessous.

| Non                                | Valeur par défaut                                                               | Description                                                                                                                                                                                                                                                          |
|-------------------------------------|-----------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `node_exporter_version`             | 1.5.0  >=                                                                   | Version du paquet de nopde-exporter.                                                                                                                                                                                                    |
| `_node_exporter_binary_install_dir` | /usr/local/bin                                                              | Dossier d'installation de l'exportateur Node                                                                                                                                                                                                                                    |
| `_node_exporter_system_group`       | node-exp                                                                    | Node exporter System Group                                                                                                                                                                                                                                           |
| `_node_exporter_system_user`        | node-exp                                                                    | Node exporter  Utilisateur du système                                                                                                                                                                                                                                            |
| `node_exporter_web_listen_address`  | "0.0.0.0:9100"                                                              | Adresse sur laquelle le node exporter écoutera                                                                                                                                                                                                                          |
| `node_exporter_web_telemetry_path`  | "/metrics"                                                                  | Chemin sous lequel les mesures doivent être exposées                                                                                                                                                                                                                                   |
| `node_exporter_enabled_collectors`  | ```["systemd",{textfile: {directory: "{{node_exporter_textfile_dir}}"}}]``` | Liste de fichiers définissant les collecteurs activés en plus et leur configuration. Il ajoute des collecteurs à [ceux activés par défaut](https://github.com/prometheus/node_exporter#enabled-by-default).                                                                    |
| `node_exporter_disabled_collectors` | []                                                                          | Liste des collecteurs désactivés. Par défaut, node_exporter désactive les collecteurs listés [ici](https://github.com/prometheus/node_exporter#disabled-by-default).                                                                                                            |
| `node_exporter_textfile_dir`        | "/var/lib/node_exporter"                                                    | Répertoire utilisé par le [Collecteur de fichiers texte](https://github.com/prometheus/node_exporter#textfile-collector). Pour obtenir les permissions d'écrire des métriques dans ce répertoire, les utilisateurs doivent être dans le groupe système `node-exp`. Note__ : Plus d'informations dans le guide TROUBLESHOOTING.md. 
| `node_exporter_tls_server_config`   | {}                                                                          | Configuration de l'authentification TLS. Les clés et les valeurs sont les mêmes que dans [node_exporter docs].(https://github.com/prometheus/node_exporter/blob/master/https/README.md#sample-config).                                                                                |
| `node_exporter_http_server_config`  | {}                                                                          | Config pour le support HTTP/2. Les clés et les valeurs sont les mêmes que dans [node_exporter docs].(https://github.com/prometheus/node_exporter/blob/master/https/README.md#sample-config).                                                                                           |
| `node_exporter_basic_auth_users`    | {}                                                                          | Dictionnaire d'utilisateurs et de mots de passe pour l'authentification de base. Les mots de passe sont automatiquement hachés avec bcrypt.                                                                                                                                                           |
| `node_exporter_dgfip_download_url`  | http://nexus3.appli.dgfip/repository/prometheus_node_exporter          | URL du node exporter binaire  dans Nexus Cloud.                                                                                                                                                                                                                          |

## Example

### Playbook

Utilisez-le dans un playbook comme suit :
```yaml
- hosts: all
  roles:
    - dgfip.node_exporter
```

### TLS config(Facultatif)

Avant d'exécuter le rôle node_exporter, l'utilisateur doit fournir son propre certificat et sa propre clé.
```yaml
- hosts: all
  pre_tasks:
    - name: Create node_exporter cert dir
      file:
        path: "/etc/node_exporter"
        state: directory
        owner: root
        group: root

    - name: Create cert and key
      openssl_certificate:
        path: /etc/node_exporter/tls.cert
        csr_path: /etc/node_exporter/tls.csr
        privatekey_path: /etc/node_exporter/tls.key
        provider: selfsigned
  roles:
    - dgfip.node_exporter
  vars:
    node_exporter_tls_server_config:
      cert_file: /etc/node_exporter/tls.cert
      key_file: /etc/node_exporter/tls.key
    node_exporter_basic_auth_users:
      randomuser: examplepassword 
```
## ChangeLog

[v1.0.13]
  * Correction ansible-lint (Issue #25)
  * Suppression du répertoire 'molecule'.

[v1.0.12]
  * Suppression du recursive sur l'arboresence `node_exporter_textfile_dir` (Issue #24)

[v1.0.11]
  * Suppression de la mention "latest" sur la version de`node_exporter_version` (Issue #23) 

[v1.0.10]
  * Meilleure gestion des collecteurs de *filesystems* ; ignorer les *filesystems* de type `fuse.rclone` (Issue #21)
  
[v1.0.9]
  * Compatibilité débian (Issue #20)
  
[v1.0.8]
  * Remplacer "nexus-cloud" par "nexus3" dans le readme
  * Amélioration de l'idempotence (Issue # 18)
  * Prise en compte des remontées de ansible-lint

[v1.0.7]
  * changement du nom de la variable dgfip_download_url

[v1.0.6]
  * compatibilité avec la version 1.6.1
  * Mise à jour du template node_exporter.service.j2
  * Mise à jour du Readme
  * Suppression de l'installation de la version latest depuis internet

[v1.0.5]
  * Ajout de l'option "CAP_DAC_READ_SEARCH" pour un accès en lecture seule sur les répertoires

[v1.0.4]
  * Correction des changements de droits et du owner de /tmp

[v1.0.2]
  * Mise à jour de la version

[v1.0.0]
  * Initial version
