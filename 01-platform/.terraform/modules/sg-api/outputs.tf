output "secgroup_name" {
  # The join() hack is required because currently the ternary operator
  # evaluates the expressions on both branches of the condition before
  # returning a value. When providing and external VPC, the template VPC
  # resource gets a count of zero which triggers an evaluation error.
  #
  # This is tracked upstream: https://github.com/hashicorp/hil/issues/50
  #
  description = "Name of the created security group (or of the existing group if specified)"

  value = join(" ", openstack_networking_secgroup_v2.sg.*.name)
}

output "secgroup_id" {
  value = join(" ", openstack_networking_secgroup_v2.sg.*.id)
}
