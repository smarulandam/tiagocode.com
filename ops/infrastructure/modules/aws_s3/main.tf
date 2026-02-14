resource "aws_s3_bucket" "s3_bucket" {
  bucket = "${var.project_name}-bucket"
  force_destroy = true
  
  tags = {
    "Name" = "${var.project_name}-bucket"    
  }
}

resource "aws_s3_bucket_public_access_block" "default_access_block" {
  bucket = aws_s3_bucket.s3_bucket.id

  block_public_acls = false
  block_public_policy = false
  ignore_public_acls = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_ownership_controls" "default_ownership_controls" {
  bucket = aws_s3_bucket.s3_bucket.id

  rule {
    object_ownership = "ObjectWriter"
  }
}

resource "aws_s3_bucket_cors_configuration" "default_cors_configuration" {
  bucket = aws_s3_bucket.s3_bucket.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET"]
    allowed_origins = ["${var.project_domain_name}", "*.${var.project_domain_name}"]
    expose_headers = []
    max_age_seconds = 300
  }
}