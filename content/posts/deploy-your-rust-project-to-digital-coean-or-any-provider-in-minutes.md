+++
title = "Deploy Your Rust Project to Digital Ocean (or Any Provider) in Minutes"
description = "Learn how to quickly deploy a Rust project to Digital Ocean (or any provider) with this streamlined guide. Assuming a Rust project running on port 3000, this method demonstrates how Rust compiles to an executable binary, eliminating the need for Docker or Rust installation on the server, and likely bypassing the need for Nginx. Just compile, upload, and run. Note: This approach may miss some best practices for large-scale production deployment."
date = 2023-08-12
+++

In this post, I will show you the fastest way to deploy a Rust project to Digital Ocean. This guide assumes you already
have a Rust project with a web server running on port 3000. The principles discussed here can also be applied to other
hosting providers. Since Rust compiles to an executable binary, you don't need Docker or have to install Rust on the
server, and you probably don't need Nginx either. Just compile, upload, and run.

## Objective:

We aim to create a script that will update the server from a local environment. While it's possible to use something
from a Continuous Integration (CI) pipeline, this post will focus on a quick deployment to allow you to get back to
building your project.

## Steps

1. Create a Regular Droplet with Debian and SSH access. Nothing too powerful is required; Rust can go a long way on
   modest resources.
2. SSH into the box and clone your repository. If you don't need any files from your repository, you can skip this step.
   In this example, we'll call the app "my_app."
3. Determine the architecture
   ```bash
   $ dpkg --print-architecture
   amd64
   ```
4. Install the [Cross](https://github.com/cross-rs/cross) crate to compile our Rust app easily for any target.
5. Compile the app with the correct architecture
   ```bash
   cross build --target x86_64-unknown-linux-gnu --release
   ```
6. Upload the binary to the Droplet
   ```bash
   scp /your-path/my_app/target/x86_64-unknown-linux-gnu/release/my_app root@24.199.110.229:/root/my_app
   ```
7. Make the binary execuatable
   ```bash
   ssh -f root@24.199.110.229 'chmod +x /root/my_app/my_app'
   ```
8. Start the app
   ```bash
   ssh -f root@24.199.110.229 './root/my_app'
   ```
9. Expose port 3000 in the Droplet's firewall rules
   {{ resize_image(path="../static/images/digital-ocean-port-3000.png", alt="Allow port 3000 in a Digital Ocean droplet firewall rule") }}

## Update Script

Here's a script to use for future updates to the Rust app.

```bash
cargo install cross
cross build --target x86_64-unknown-linux-gnu --release

ssh -f root@24.199.110.229 'sudo kill $(sudo lsof -t -i :3000)'
ssh -f root@24.199.110.229 'cd my_app && git pull'
scp /your-path/my_app/target/x86_64-unknown-linux-gnu/release/my_app root@24.199.110.229:/root/my_app
ssh -f root@24.199.110.229 'chmod +x /root/my_app/my_app'
ssh -f root@24.199.110.229 './root/my_app'
```

## Improvements

While this method is a quick way to deploy an app and return to building, it is worth noting that it misses some
best practices for deploying a production application designed for a large user base.