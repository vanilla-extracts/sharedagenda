# Define required providers
terraform {
  required_version = ">= 0.14.0"
  required_providers {
    openstack = {
      source = "terraform-provider-openstack/openstack"
      #version = "> 1.53.0"
    }
  }
}

# Configure the OpenStack Provider
provider "openstack" {
  cloud = "openstack"

  #  user_name   = "admin"
  #  tenant_name = "admin"
  #  password    = "pwd"
  #  auth_url    = "http://myauthurl:5000/v3"
  #  region      = "RegionOne"
}
