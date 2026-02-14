output "instance_id" {
  value = aws_instance.ec2_instance.id
}

output "instance_name" {
  value = aws_instance.ec2_instance.tags["Name"]
}

output "instance_public_ip" {
  value = aws_eip.eip.public_ip
}

output "instance_private_key_pem_content" {
  value = tls_private_key.rsa_private_key.private_key_pem
}