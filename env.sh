#! /bin/bash
# Auteur: Yann LE GUENNEC <yann.le-guennec@dgfip.finances.gouv.fr>
# Maintainer: Charlotte THOMAS <charlotte.thomas@dgfip.finances.gouv.fr>
# Log :
#   18/04/2025 - Adapt Script for SharedAgenda deployment
#   01/04/2025 - Change ssh keys for our group
#   19/12/2024 - Renommage plateform_name -> pf_prefixe
#   14/06/2023 - init.sh remplace env.sh
#   10/06/2022 - Récupère le numéro de plateforme en paramètre
#   01/06/2022 - Première version

# Vérifie qu'on utilise bien la commande 'source'
[[ "$0" == "$BASH_SOURCE" ]] && echo "Vous devez utiliser la commande : source $BASH_SOURCE"
[[ "$0" == "$BASH_SOURCE" ]] && exit 1

[[ ! -f $PWD/env.sh ]] && echo "Vous devez utiliser le fichier '$(basename $BASH_SOURCE)' dans le répertoire où il se trouve."
[[ ! -f $PWD/env.sh ]] && return

# Variables globales
version=20250424
python='env python3'
nbMaxProjects=30
varsFile=./.env_vars
bucketFile=./01-platform/tfstate.tf
sshPem=~/.ssh/id_sharedagenda
virtualEnv=.Nubo
proxyHost=proxy.infra.dgfip
proxyPort=3128
nubo=01
cloud=${OS_CLOUD:-openstack}

function readVars
{
    # Recupère les informations, si elle existent
    echo
    echo "Projet Nubo"
    echo "-----------"

    [[ -f "${varsFile}" ]] && {
        echo "  - Variables : ${varsFile}"
        source "${varsFile}"
    }
    echo "  - Cloud : ${cloud}"
}

function askInfos
{
    # Demande les information de la plateforme si elle ne sont pas disponibles

    # Numéro de la plateforme
    [[ -n $1 ]] && platform_id=$1
    [[ -z ${platform_id} ]] &&
        read -p "  - Numéro de la plateforme (1..${nbMaxProjects}) : " platform_id

    # Vérification de l'entrée
    while [[ ! ${platform_id} =~ ^[0-9]*$ ]] || (( $(( 10#${platform_id} )) < 1 || $(( 10#${platform_id} )) > ${nbMaxProjects} ))
    do
        echo "*** Erreur: Entrez un numéro de projet entre 1 et ${nbMaxProjects}"
        read -p "  - Numéro de la plateforme : " platform_id
    done
    platform_id=$(printf "%02d" $(( 10#${platform_id} )) )

    # Nom de la plateforme
    [[ -z ${platform} ]] && {
        platform="pf${platform_id}"
        read -p "  - Nom de la plateforme (trigramme ou quadrigramme alphanumérique) [${platform}]: " -i "${platform}" reply
        [[ -n ${reply} ]] && platform=${reply}
    }

    # Vérification de l'entrée (Conforme aux exigences A2C)
    while [[ ! ${platform} =~ ^[a-z0-9]{3,4}$  ]]
    do
        echo "*** Erreur: Le nom doit être un trigramme ou un quadrigramme alphanumérique"
        read -p "  - Nom de la plateforme [${platform}]: " -i "${pf_prefixe}" reply
        [ -n ${reply} ] && platform=${reply}
    done

    pf_prefixe="${platform}-${nubo}"

    echo "  - Plateforme : ${pf_prefixe} (Id ${platform_id})"

    echo
    echo "Projet GitLab (stockage du fichier tfstate)"
    echo "-------------------------------------------"

    # Identifiant du projet GitLab
    [[ -n "$2" ]] && gitlab_project_id=$2
    [[ -n "${gitlab_project_id}" ]] &&
        echo "  - Identifiant numérique projet GitLab : ${gitlab_project_id}" ||
        read -p "  - Identifiant numérique projet GitLab : " -i "${gitlab_project_id}" gitlab_project_id

    while [[ ! ${gitlab_project_id} =~ ^[0-9]*$  ]]
    do
        echo "*** Erreur: L'identifiant de projet doit être numérique."
        read -p "  - Identifiant numérique projet GitLab : " -i "${gitlab_project_id}" gitlab_project_id
    done

    # Utilisateur GitLab
    [[ -n "$3" ]] && gitlab_username=$3
    [[ -n "${gitlab_username}" ]] &&
        echo "  - Utilisateur GitLab : ${gitlab_username}" ||
        read -p "  - Utilisateur GitLab : " -i "${gitlab_username}" gitlab_username

    # Jeton applicatif GitLab
    [[ -n "$4" ]] && gitlab_apptoken=$4
    [[ -z "${gitlab_apptoken}" ]] && {
#        echo -e '    Création du jeton applicatif : https://forge.dgfip.finances.rie.gouv.fr/yann.le-guennec/devops/-/settings/access_tokens (Ctrl-Click)'
        echo -e "    Création du jeton applicatif : Dans le projet GitLab > Paramètres > Jetons d'accès > Ajouter un nouveau jeton"
        read -sp "  - Jeton applicatif GitLab : " gitlab_apptoken
    }

    echo
}

# Génère une clef SSH pour les machines virtuelles sur Nubo
function createSSHKey
{
    # Ne regénère jamais la clef si ell e existe déjà
    [[ -f "${sshPem}" ]] || {
        # Génération d'une nouvelle clef ssh
        echo "  - Génération de la clef ssh pour ${pf_prefixe}"
        ssh-keygen -C "${pf_prefixe}" -f "${sshPem}" -m pem -t rsa -N ""
    }
}

# Crée le fichier env.sh qui met en place tout l'environnement de travail
function createVars
{
    # Création du fichier des variables
    cat > "${varsFile}" <<EOF
nubo=${nubo}
cloud=${cloud}
platform_id=${platform_id}
platform=${platform}
pf_prefixe=${pf_prefixe}
gitlab_project_id=${gitlab_project_id}
gitlab_username="${gitlab_username}"
gitlab_apptoken="${gitlab_apptoken}"
EOF
}

function createBucket
{
    # La ressource "terraform" ne supporte pas les variables, il faut donc recréer le fichier en entier.
    cat > "${bucketFile}" <<EOF
# Création du jeton applicatif
# https://forge.dgfip.finances.rie.gouv.fr/-/profile/personal_access_tokens?name=Test&scopes=api

terraform {
  backend "http" {
    address="https://forge.dgfip.finances.rie.gouv.fr/api/v4/projects/${gitlab_project_id}/terraform/state/${pf_prefixe}-state"
    lock_address="https://forge.dgfip.finances.rie.gouv.fr/api/v4/projects/${gitlab_project_id}/terraform/state/${pf_prefixe}-state/lock"
    unlock_address="https://forge.dgfip.finances.rie.gouv.fr/api/v4/projects/${gitlab_project_id}/terraform/state/${pf_prefixe}-state/lock"
    lock_method="POST"
    unlock_method="DELETE"
    retry_wait_min="5"
  }
}
EOF
}


function setupEnv
{
    echo "Mise en place de l'environnement"
    echo "--------------------------------"

    export platform_id=${platform_id}
    export pf_prefixe=${pf_prefixe}

    # Configuration pour openstack
    export OS_CLOUD=${cloud}

    # Configuration pour Terraform
    export TF_VAR_nubo=${cloud}
    export TF_VAR_cloud="${cloud}"
    export TF_VAR_platform_id="${platform_id}"
    export TF_VAR_pf_prefixe="${pf_prefixe}"
    export TF_VAR_key_pair="${sshPem}.pub"

    export TF_HTTP_USERNAME="${gitlab_username}"
    export TF_HTTP_PASSWORD="${gitlab_apptoken}"

    # Configure le proxy, sauf pour rie.gouv.fr et dgfip
    export http_proxy=http://$proxyHost:$proxyPort
    export https_proxy=http://$proxyHost:$proxyPort
    export no_proxy=rie.gouv.fr,dgfip,localhost
}

function setupSSH
{
    # Demarrage de l'agent SSH et ajout de la clef ssh
    [[ -z "${SSH_AGENT_PID}" ]] && echo -n "  - SSH " && eval $(ssh-agent -s 2> /dev/null)
    echo -n "  - SSH " && ssh-add "${sshPem}"
}

function setupPythonEnv
{
    # Activation du virtualEnv Python
    [[ -f ${virtualEnv}/bin/activate ]] ||
    {
        echo "  - Création de l'environnement virtuel Python"
        ${python} -m venv ${virtualEnv}
    }

    [[ -f ${virtualEnv}/bin/activate ]] &&
    {
        echo "  - Activation de l'environnement virtuel Python"
        . ${virtualEnv}/bin/activate
    } ||
    {
        echo "*** L'environnement virtuel Python n'a pas pu être activé."
    }
}

function setupAliases
{
    # Ajoute quelques alias
    echo "  - Aliases"
    alias tf=terraform
    alias ap='ansible-playbook -i inventory'
    alias os=openstack
}

# Programme principal

echo
echo "Cours DevOps (Version ${version})"
echo "============"

readVars
askInfos "$@"

createVars
createSSHKey
createBucket

setupEnv
setupSSH
setupAliases
setupPythonEnv

echo
