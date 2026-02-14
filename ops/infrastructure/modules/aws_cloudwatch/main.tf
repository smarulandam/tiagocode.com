resource "aws_cloudwatch_dashboard" "app_dashboard" {
  dashboard_name = "${var.project_name}-dashboard"
  dashboard_body = jsonencode({
    widgets = [{
        type   = "metric"
        x      = 0
        y      = 0
        width  = 12
        height = 6
        properties = {
          metrics = [
            ["AWS/EC2", "CPUUtilization", "InstanceId", "${var.ec2_instance_id}"]
          ]
          period = 86400
          stat   = "Average"
          region = var.aws_region
          title  = "EC2 ${var.ec2_instance_id} - CPU Utilization"
        }
      }, {
        type   = "metric"
        x      = 0
        y      = 6
        width  = 6
        height = 6
        properties = {
          view: "timeSeries",
          metrics: [
              ["AWS/EC2", "EBSWriteOps", "InstanceId", var.ec2_instance_id ]
          ],
          region: var.aws_region,
          period: 300,
          stacked: true,
          title = "EC2 ${var.ec2_instance_id} - EBSWriteOps"
        }
      }, {
        type   = "metric"
        x      = 6
        y      = 6
        width  = 6
        height = 6
        properties = {
          view: "timeSeries",
          metrics: [
              ["AWS/EC2", "EBSReadOps", "InstanceId", var.ec2_instance_id ]
          ],
          region: var.aws_region,
          period: 300,
          stacked: true
          title = "EC2 ${var.ec2_instance_id} - EBSReadOps"
        }
      }, {
        type   = "metric"
        x      = 0
        y      = 12
        width  = 12
        height = 6
        properties = {
          metrics = [
            ["AWS/EC2", "NetworkIn", "InstanceId", "${var.ec2_instance_id}"]
          ]
          period = 300
          stat   = "Average"
          region = var.aws_region
          title  = "EC2 ${var.ec2_instance_id} - NetworkIn"
        }
      }, {
        type   = "metric"
        x      = 12
        y      = 0
        width  = 12
        height = 6
        properties = {
          view: "timeSeries",
          metrics: [
              [ "AWS/RDS", "DatabaseConnections", "DBInstanceIdentifier", "${var.rds_instance_id}"],
              [ "AWS/RDS", "ReadIOPS", "DBInstanceIdentifier", "${var.rds_instance_id}"],
              [ "AWS/RDS", "WriteIOPS", "DBInstanceIdentifier", "${var.rds_instance_id}"]
          ],
          region: var.aws_region
          period: 86400
          title: "RDS ${var.rds_instance_id} - DatabaseConnection & Read/Write IOPS"
        }
      }, {
        type   = "metric"
        x      = 12
        y      = 6
        width  = 12
        height = 6
        properties = {
          view: "singleValue",
          stacked: true,
          sparkline: true,
          metrics: [
              [ "AWS/S3", "NumberOfObjects", "BucketName", var.bucket_id, "StorageType", "AllStorageTypes"],
              [ "AWS/S3", "BucketSizeBytes", "BucketName", var.bucket_id, "StorageType", "StandardStorage"]
          ],
          region: var.aws_region
          period: 604800
          title: "S3 ${var.bucket_id} - NumberOfObjects & BucketSizeBytes"
        }
      }]
  })
}
