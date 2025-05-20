# Création du jeton applicatif
# https://forge.dgfip.finances.rie.gouv.fr/-/profile/personal_access_tokens?name=Test&scopes=api

terraform {
  backend "http" {
    address="https://forge.dgfip.finances.rie.gouv.fr/api/v4/projects/38485/terraform/state/shag-01-state"
    lock_address="https://forge.dgfip.finances.rie.gouv.fr/api/v4/projects/38485/terraform/state/shag-01-state/lock"
    unlock_address="https://forge.dgfip.finances.rie.gouv.fr/api/v4/projects/38485/terraform/state/shag-01-state/lock"
    lock_method="POST"
    unlock_method="DELETE"
    retry_wait_min="5"
  }
}
