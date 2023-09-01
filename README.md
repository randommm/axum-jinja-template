## About

You can use this repo as a template for Axum and with Minijinja used as the HTML template system.

Moreover, a deployment example with GitHub Actions is provided.

## Conventional setup

* If you don't have `Rust` installed, see `https://rustup.rs`.

* Deploy with `cargo run --release`, then just browse your website at `http://localhost:3511`.

## Setup with Docker Compose

* Deploy with `docker-compose up`, then just browse your website at `http://localhost:3511`.

## Setup with Docker (but without Docker Compose)

* Build your OCI (Docker image) with `docker build -t ghcr.io/randommm/axum-jinja-template .`.

* Deploy with `docker run --rm -p 3511:3511 ghcr.io/randommm/axum-jinja-template`, then just browse your website at `http://localhost:3511`.

## Optional extra: production deploy with Nginx

You can additional deploy on Nginx by adding the following to its configuration file (generally located at `/etc/nginx/sites-available/default`):

      server {
            server_name youdomainname.com;
            listen 80;
            location / {
                  proxy_pass http://127.0.0.1:3511;
                  proxy_set_header        Host $host;
            }
      }

Additionally, you can obtain an SSL certificate by running `certbot --nginx`

## Optional extra: automate deploy Github Actions

Now, place `docker-compose.yml` on your server along with the `.env` and edit the `docker-compose.yml`, change:

      container_name: axum-jinja-template
      build:
          context: .

to:

      container_name: axum-jinja-template
      image:  ghcr.io/put_your_github_username_here/axum-jinja-template

Finally, edit your crontab (`crontab -e`) to auto check, pull and deploy changes every 5 minutes:

      */5 * * * * cd /folder_where_docker-compose.yml_is_located/ && /usr/bin/docker-compose pull && /usr/bin/docker-compose up -d
