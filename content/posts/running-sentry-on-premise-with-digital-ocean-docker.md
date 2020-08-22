+++
title = "Running Sentry On-Premise with Digital Ocean & Docker"
description = "Running Sentry On-Premise with Digital Ocean & Docker"
date = 2019-12-10
+++


    
This article will go through the steps needed to get Sentry On-Premise working on a Digital Ocean preconfigured
Docker droplet with TLS. Running Sentry yourself will eliminate bandwidth costs because your droplets will be
sending data through Digital Ocean's network rather than across the internet. It will potentially cost less
because a droplet with minimum system requirements is $15 a month versus the $26 team tier available from
Sentry.
    
## Installing Sentry On-Premise

Start by creating a new Digital Ocean droplet. It must have at least 3 gigs of RAM and make sure the "Private
networking" option is selected. Once the droplet is created then ssh as root into the droplet.
    
```shell script
ssh root@your-droplet-ip-address
```

**Notice:** This article is setting up everything on the droplet as the root user. This is not a
good practice and that is outside the scope of this article.
    
Create a new folder which will be used as the install location for Sentry On-Premise
    
```shell script
$ mkdir /home/root/ && mkdir /home/root/apps
```    
   
Ensure that you are in <code>/home/root/apps</code> and clone the Sentry On-Premise
<a target="_blank" href="https://github.com/getsentry/onpremise">Github</a> Repository. Next,
generate a secret required for Sentry On-Premise. Inside the onpremise directory (created from git
clone) run the following command to generate a new secret.
   
```shell script
docker run sentry config generate-secret-key
```
    
    
Docker will fetch the images needed to generate the secret and a new secret will be displayed in the terminal.
Copy it and we'll add it to the config file. A config file can be created by copying the existing example
config.
    
```shell script
cp ./sentry/config.example.yml ./sentry/config.yml
```
    
    
Open the new config.yml file, find <code>system.secret-key: '!!changeme!!'</code>, and replace "!!changeme!!"
with the secret generated from before. That's it for configuring sentry! Next, run the install script to install
all of the dependencies. It will prompt to create a user account which will be used to login into the sentry
dashboard.
    
```shell script
. ./install.sh
```
## Running Sentry
    
Once everything has installed, run <code>docker-compose up -d</code> to get everything up and running.
You won't be able to visit the sentry dashboard yet because port 9000 is blocked by default on droplets. You
can open the port by running <code>ufw allow 9000</code>. You can now visit http://doplet-ip-address:9000
and should see the Sentry login screen! However, it's not a good security practice to send error information
without TLS. In the next section, NGINX will be used to handle TLS and proxy traffic through port 80 to port
9000. For now, disable port 9000 <code>ufw deny 9000</code>.
    
## Configuring and Running NGINX

First, start off my updating package indexes <code>apt-get update</code> and then install NGINX
<code>apt-get install nginx-full</code>.
    
NGINX needs to be configured correctly to know which certificate and key to look for and which port should be
forwarded to 80. Edit <code>/etc/nginx/nginx.config</code> and copy/paste the code below into it. All instances
of of "website-domain.com" to the correct domain name. The certificate and private key of the domain will need
to
be added in the locations as specified under the SSL Configuration section.

```nginx
events {}

http {
  # set REMOTE_ADDR from any internal proxies
  # see http://nginx.org/en/docs/http/ngx_http_realip_module.html
  set_real_ip_from 127.0.0.1;
  set_real_ip_from 10.0.0.0/8;
  real_ip_header X-Forwarded-For;
  real_ip_recursive on;

  # SSL configuration -- change these certs to match yours
  ssl_certificate      /etc/ssl/website-domain.com.crt;
  ssl_certificate_key  /etc/ssl/website-domain.com.key;

  # NOTE: These settings may not be the most-current recommended
  # defaults
  ssl_protocols TLSv1 TLSv1.1 TLSv1.2;
  ssl_ciphers ECDH+AESGCM:DH+AESGCM:ECDH+AES256:DH+AES256:ECDH+AES128:DH+AES:ECDH+3DES:DH+3DES:RSA+AESGCM:RSA+AES:RSA+3DES:!aNULL:!MD5:!DSS;
  ssl_prefer_server_ciphers on;
  ssl_session_cache shared:SSL:128m;
  ssl_session_timeout 10m;

  server {
    listen   80;
    server_name website-domain.com;

    location / {
      if ($request_method = GET) {
        rewrite  ^ https://$host$request_uri? permanent;
      }
      return 405;
    }
  }

  server {
    listen   443 ssl;
    server_name website-domain.com;

    proxy_set_header   Host                 $http_host;
    proxy_set_header   X-Forwarded-Proto    $scheme;
    proxy_set_header   X-Forwarded-For      $remote_addr;
    proxy_redirect     off;

    # keepalive + raven.js is a disaster
    keepalive_timeout 0;

    # use very aggressive timeouts
    proxy_read_timeout 5s;
    proxy_send_timeout 5s;
    send_timeout 5s;
    resolver_timeout 5s;
    client_body_timeout 5s;

    # buffer larger messages
    client_max_body_size 5m;
    client_body_buffer_size 100k;

    location / {
      proxy_pass        http://localhost:9000;

      add_header Strict-Transport-Security "max-age=31536000";
    }
  }
}
```

    
Locking down the ports and opening the correct ones is the last step. Allow https, http and port
22, but otherwise block everything else. For whatever reason, ports 2376 and 2375 are open on the docker
droplet. Run the following command to correctly configure the ports.
    
```shell script
ufw allow https && \
  ufw allow http && \
  ufw deny 2375 && \
  ufw deny 2375/tcp && \
  ufw deny 2376  && \
  ufw deny 2376/tcp
```    

Restart nginx <code>systemctl restart nginx</code> for everything to take effect. Visit
<a target="_blank" href="https://website-domain.com">https://website-domain.com</a> to see the Sentry dashboard!
    