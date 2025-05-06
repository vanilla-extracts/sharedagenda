
# Description

Exportateur Prometheus Postgres.

Ceci utilise actuellement le compte administrateur local `postgres` sur le serveur PostgreSQL.

## Exigences

- Ansible >= 2.7 (Il est possible qu'il fonctionne sur des versions antérieures, mais nous ne pouvons pas le garantir)

## Les variables du Rôle 

Toutes les variables qui peuvent être modifiées sont stockées dans le fichier [defaults/main.yml](defaults/main.yml) ainsi que dans le tableau ci-dessous.
| Nom                                 | Valeur par défaut                                                                                    | Description                                                                                                                                                            |
|--------------------------------------|--------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `postgres_exporter_version`          | 0.17.1                                                                                            | Version du paquet exportateur Postgres                                                                                                                                      |
| `postgres_exporter_bin_dir`          | /usr/local/bin                                                                                   | Dossier d'installation de Postgres-exporter                                                                                                                                  |
| `postgres_exporter_dbname`           | postgres                                                                                         | Le nom de la base de données                                                                                                                                                      |
| `postgres_exporter_data_source_name` | "user=postgres dbname=\{{ postgres_exporter_dbname }} host=/var/run/postgresql/ sslmode=disable" | implies `DATA_SOURCE_NAME` ENV variable in the postgres_exporter                                                                                                       |
| `postgres_exporter_system_group`     | postgres                                                                                | Groupe secondaire à rattacher à l'utilisateur     `postgres_exporter_system_user` |
| `postgres_exporter_system_user`      | postgres                                                                                        | Utilisateur système de Postgres Exporter                                                                                                                                         |
| `postgres_exporter_listen_address:postgres_exporter_port`            | 0.0.0.0:9187                                                                                             | Port du Postgres Exporter                                                                                                                                                  |
| `dgfip_download_url`                 | https://nexus3.appli.dgfip/repository/prometheus_postgres_exporter                           | URL du binaire postgres exporter dans Nexus.                                                                                                                        |
| `postgres_exporter_flags_collector`                 | "" | Liste des collecteurs à activer/désactiver. On peut ajouter des flags "[no-]collector.database [no-]collector.database_wraparound   |
| `postgres_pg_exporter_extend_query_path`  | /etc/prometheus/queries                                                                             | Dossier pour les requêtes personnalisées|
| `postgres_pg_exporter_extend_queries`  | Exemple de requêtes sql                                                                                           | Les requêtes personnalisées                                                                                             |

Exemple de possibilité d'adapter la chaîne de connexion à la base de données :

```yaml
postgres_exporter_data_source_name: "postgresql://utilisateur:motdepasse@localhost:5432/postgres?sslmode=disable"
```
```yaml
- Suppression des requetes via fichiers yaml au profit des requêtes reprises dans les flags collector fournis par l’exporter (#Issue11)
- Suppression de l’option –extend.query-path
- Création d'un répertoire "postgres_pg_exporter_extend_query_path" s'il n'existe pas" afin d'ajouter le custom file 
- Ajout de la variable d'environnement PG_EXPORTER_EXTEND_QUERY_PATH pour pemettre l’ajout des requêtes custom

```

# ChangeLog

## [v1.2.0]
- Suppression des requetes via fichiers yaml au profit des requêtes reprises dans les flags collector fournis par l’exporter (#Issue11)
- Suppression de l’option –extend.query-path
- Ajout de la variable d’environnement PG_EXPORTER_EXTEND_QUERY_PATH pour permettre l’ajout des requêtes custom  

## [v1.1.2]
- Mise à jour de la version par défaut : 0.17.1
- Ajout de la variable « postgres_exporter_flags_collector ». Possiblilité de désactiver ou activer certaines métrics (Issue #10)
  Exemple:
 ```yaml
 postgres_exporter_flags_collector: "--collector.postmaster --collector.stat_bgwriter --collector.locks --no-collector.database_wraparound"
``` 

## [v1.1.1]
- Fixer la valeur de `postgres_exporter_system_user` à 'postgres' afin de pouvoir se connecter en mode 'peer'.

## [v1.1.0]
- Adaptations debian (Issue #7)
- Check post-installation

## [v1.0.6]
- Revue documentaire (Issue #6)

## [v1.0.5]

- Mise en conformité du Readme

## [v1.0.4]

- Fix linter ansible (Issue #4)
- Meilleure idempotence du rôle (pas de réinstallation en cas de rejeu)

## [v1.0.3] 

- Default version to Deploy is now 0.14.0
- Change default url for downloading the component (nexus-cloud --> nexus3)
- Remove 'molecule' folder

## [v1.0.2] - DGFIP - Thomas Moinelet (CADOS)

- Surcharge du mécanisme SystemD
- Fix pour éviter la modification du shell de l'utilisateur 'postgres'

## Example

### Playbook

```yaml
---
    - hosts: localhost
      roles:
      - role: dgfip.postgres_exporter
        postgres_exporter_dbname: test
```
