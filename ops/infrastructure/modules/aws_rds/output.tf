output "instance_address" {
  value = aws_db_instance.database_instance.address
}

output "instance_id" {
  value = "${var.project_name}-db-instance"
}