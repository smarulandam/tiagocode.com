data "aws_rds_orderable_db_instance" "postgres" {
  engine = var.instance_engine
  engine_version = var.instance_engine_version
  storage_type = var.instance_storage_type
  preferred_instance_classes = [var.instance_class]
}

resource "aws_db_subnet_group" "default_subnet_group" {
  name = "${var.project_name}-subnet-group"
  subnet_ids = var.private_subnets_ids
  
  tags = {
    Name = "${var.project_name}-subnet-group"
  }
}

resource "aws_db_instance" "database_instance" {
  availability_zone = "eu-north-1a"
  identifier = "${var.project_name}-db-instance"
  allocated_storage = 20
  engine = data.aws_rds_orderable_db_instance.postgres.engine
  engine_version = data.aws_rds_orderable_db_instance.postgres.engine_version
  instance_class = data.aws_rds_orderable_db_instance.postgres.preferred_instance_classes[0]
  storage_type = data.aws_rds_orderable_db_instance.postgres.storage_type  
  db_subnet_group_name = aws_db_subnet_group.default_subnet_group.name
  db_name = var.database_name
  username = var.database_username
  password = var.database_password
  multi_az = false
  apply_immediately = true
  skip_final_snapshot = true

  vpc_security_group_ids = var.security_groups_ids
}