#! /bin/bash
# Auteur: Yann LE GUENNEC <yann.le-guennec@dgfip.finances.gouv.fr>
# Maintainer: Charlotte THOMAS <charlotte.thomas@dgfip.finances.gouv.fr>
# Log :
#   14/05/2025 - Massive refactoring for home usage
#   18/04/2025 - Adapt Script for SharedAgenda deployment
#   01/04/2025 - Change ssh keys for our group
#   19/12/2024 - Renommage plateform_name -> pf_prefixe
#   14/06/2023 - init.sh remplace env.sh
#   10/06/2022 - Récupère le numéro de plateforme en paramètre
#   01/06/2022 - Première version

# Vérifie qu'on utilise bien la commande 'source'
python='env python3'
nbMaxProjects=30
sshPem=~/.ssh/id_github_me.pub
virtualEnv=.Home



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
    alias ap='ansible-playbook -i inventory'
}

# Programme principal

setupEnv
setupSSH
setupAliases
setupPythonEnv

echo
