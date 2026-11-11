---
id = "doc/vendors/aws/aws"
kind = "vendor"
title = "Amazon Web Services (AWS)"
category = "reference/vendors/aws"
tags = ["aws", "ec2", "cli", "networking", "model-doc"]
risk = "medium"
version = "AWS CLI v2"
---
The complete Amazon Web Services (AWS) document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Commands
<!-- section-tags: aws, ec2, cli, networking -->


### aws ec2 describe-instances

Reads EC2 instance details, including network interfaces, IPs and VPC placement.

#### Command

```bash
aws ec2 describe-instances --filters "Name=instance-state-name,Values=running"
aws ec2 describe-instances --query "Reservations[*].Instances[*].[InstanceId,PrivateIpAddress,State.Name]" --output table
```

#### Notes

- Networking data (VPC, subnet, ENIs, security groups) is under each instance's `NetworkInterfaces`.
- `--filters` narrows by state, VPC, tag and more; `--query` shapes the output.
- `--output table|json|text|yaml` changes formatting; `--max-results`/`--next-token` handle pagination.
- Read-only and safe to run in production accounts.

Source: AWS CLI Command Reference (Amazon EC2) — https://docs.aws.amazon.com


### aws ec2 modify-network-interface-attribute

Changes attributes of an EC2 network interface, such as source/dest check or attached security groups.

#### Command

```bash
aws ec2 modify-network-interface-attribute --network-interface-id eni-0123abcd --no-source-dest-check
aws ec2 modify-network-interface-attribute --network-interface-id eni-0123abcd --groups sg-0123abcd
```

#### Notes

- Disabling source/dest check is required for NAT/routing instances; re-enable with `--source-dest-check`.
- `--groups` replaces the security groups attached to the ENI - list every group you want to keep.
- Applies immediately, no reboot needed; the ENI ID is shown in the instance's networking tab.
- Changes security posture and routing behavior - double-check the ENI before running.

Source: AWS CLI Command Reference (Amazon EC2) — https://docs.aws.amazon.com

