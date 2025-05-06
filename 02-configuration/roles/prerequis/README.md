# Rôle prérequis


## Objectif

Ajouter les éventuels prérequis manquants ainsi que l'installation SELinux pour les images Debian.

## Versions
[v1.0.0]
 * Mettre  la variable force_update_cache à true (issue #11)

[v0.0.4]
 * Ajout du paramètre `post_reboot_delay` (défaut 10) au *handler* du *reboot* (issue #7)

[v0.0.3]
 * Ajout de la possbilité de rafraîchir le cache apt avec la varibale `force_update_cache` de tyope boolean (par défaut non-activé ; issue #3)

[v0.0.2]
 * Intégration du rôle SELinux (v0.0.1)

[v0.0.1]
 * Version initiale
