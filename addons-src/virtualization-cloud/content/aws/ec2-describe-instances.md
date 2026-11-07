---
id = "cmd/aws/ec2-describe-instances"
kind = "command"
title = "aws ec2 describe-instances"
category = "reference/cloud/aws"
tags = ["aws", "ec2", "cli", "networking"]
risk = "info"
version = "AWS CLI v2"
---
Reads EC2 instance details, including network interfaces, IPs and VPC placement.

## Command

```bash
aws ec2 describe-instances --filters "Name=instance-state-name,Values=running"
aws ec2 describe-instances --query "Reservations[*].Instances[*].[InstanceId,PrivateIpAddress,State.Name]" --output table
```

## Notes

- Networking data (VPC, subnet, ENIs, security groups) is under each instance's `NetworkInterfaces`.
- `--filters` narrows by state, VPC, tag and more; `--query` shapes the output.
- `--output table|json|text|yaml` changes formatting; `--max-results`/`--next-token` handle pagination.
- Read-only and safe to run in production accounts.

Source: AWS CLI Command Reference (Amazon EC2) — https://docs.aws.amazon.com
