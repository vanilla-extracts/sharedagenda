resource "openstack_compute_keypair_v2" "ssh_keypair" {
  name = "${var.pf_prefixe}-ssh"
  public_key = file("${var.home_directory}/.ssh/id_sharedagenda.pub")
}
