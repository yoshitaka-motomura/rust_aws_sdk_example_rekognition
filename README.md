# AWS SDK for Rust Example
This example demonstrates how to use the AWS SDK for Rust

## Overview

The AWS SDK for Rust was made generally available a while ago, so I decided to try it out.
I'm trying to get a photo label using rekognition.

I am getting it from an object located in S3.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html)

### Demo CLI

> **Note**
> I made a CLI for a demo using aws-sdk-rust.
> It is only using the credentials from aws-cli, so if you want to try it out, please install AWS CLI on your own as well.

#### Build
```bash
cargo build --release --bin rekognition
# Somewhere along your path
cp target/release/rekognition /usr/local/bin

```

```bash
rekognition detect <path>
```