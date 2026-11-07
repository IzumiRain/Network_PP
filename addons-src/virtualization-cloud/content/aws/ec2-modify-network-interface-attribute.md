---
id = "cmd/aws/ec2-modify-network-interface-attribute"
kind = "command"
title = "aws ec2 modify-network-interface-attribute"
category = "reference/vendors/aws"
tags = ["aws", "ec2", "cli", "networking"]
risk = "medium"
version = "AWS CLI v2"
---
Changes attributes of an EC2 network interface, such as source/dest check or attached security groups.

## Command

```bash
aws ec2 modify-network-interface-attribute --network-interface-id eni-0123abcd --no-source-dest-check
aws ec2 modify-network-interface-attribute --network-interface-id eni-0123abcd --groups sg-0123abcd
```

## Notes

- Disabling source/dest check is required for NAT/routing instances; re-enable with `--source-dest-check`.
- `--groups` replaces the security groups attached to the ENI - list every group you want to keep.
- Applies immediately, no reboot needed; the ENI ID is shown in the instance's networking tab.
- Changes security posture and routing behavior - double-check the ENI before running.

Source: AWS CLI Command Reference (Amazon EC2) — https://docs.aws.amazon.com
