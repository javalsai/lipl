<img src="assets/img/logo.png" align="right" width="33%"/>

# LIPL
> A **l**ightweight **IP L**ogger in rust

A simple IP logger perfect for machines with low resources.

# Features
* Simple and Lightweight
* Allows you to perform [redirects](#redirects)

# Index
- [LIPL](#lipl)
- [Features](#features)
- [Index](#index)
- [Usage](#usage)
- [Installation](#installation)
    - [Download automatic builds from releases](#download-automatic-builds-from-releases)
    - [Build Package for current architecture](#build-package-for-current-architecture)
    - [Build package for other architecture](#build-package-for-other-architecture)
- [Config](#config)
- [Other](#other)
    - [Redirects](#redirects)
    - [HTTPS](#https)

# Usage
```
lipl [address?] [port]
 * Address is 0.0.0.0 by default
```
This will create a directory called `log` where all logged IPs and data will be registered.

# Installation
### Download automatic builds from [releases](https://github.com/DefendSec/light-ip-logger-rs/releases)
(This is yet not implemented)

### Build Package for current architecture
Follow this if you want to build for your current architecture.
```bash
cargo build --release # Or `cargo b -r`
```
This will install all necessary dependencies and place the final binary on `cargo/debug/lipl`.

### Build package for other architecture
This can be easily done through `nix`, which will make the perfect isolated environment to produce binaries other architectures. (Remember that you can use `nix` outside of NixOS)

Currently we "only" have a config for:
* `x86_64`: *Disabled (broken)* (use cargo instead, you might be using `x86_64`)
* `aarch64`: Same as `ARMv8` I think
* `riscv64`
* `riscv32`: *Disabled (broken)*
* `i686`: Same as `x86`/`32bits` I think
* `armv7a`
* `armv7a-hf`: Perfect for a `Raspberry Pi 2 B`

`*` If you manage to make another architecture to work just [open an issue](https://github.com/DefendSec/light-ip-logger-rs/issues/new).

To compile for any of these architectures run the script `./build.sh` on the root of the project with the architecture as argument, it will check for everything and guide you through. You can obtain some more info about this by reading it.

NOTE: You can also use `other` folowed by nix's `crossSystem.config` and rust's target to try to use, example: `./build.sh other armv7a-unknown-linux-gnueabihf armv7-unknown-linux-gnueabi`

The script tries to automatically patch the binary's elf to change the interpreter into an appropiate one, but it's likely to fail or set a unappropiate one, you can patch it yourself by doing:

```bash
# Get the default interpreter for the final machine (read elf of other programs)
# In my case it's /lib/ld-linux.so.3 (-> /lib/ld-linux-armhf.so.3), on a RPi 2B
# Replace it with the one of your case
patchelf --remove-rpath --set-interpreter /lib/ld-linux.so.3 result/bin/lipl
```

You can optionally test the binary with `qemu` by doing:

```bash
# In this case for an ARM architecture and 8080 as port
qemu-arm lipl 8080
```

# Config
This is not yet supported, but it will just be a `toml` file that will allow you to edit log paths, redirect behavior, default response...
```toml
# Default TOML
[nothing]
somthing = false
```

# Other

### Redirects
This is a special route that will put the user through a page that will run JS (obfuscated) to grab things like screen size, clipboard<sup>\*</sup> and geolocation<sup>\*</sup>.

<sup>\*</sup> Permission must be enabled in browser

If JS is not enabled it will quickly refresh page and perform the redirect server-side.

Redirects can be done by creating a link like `/rd/<url>`, where `url` is the target url reversed and [URI component encoded](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent). (`encode(url.reverse())`)


### HTTPS
Most browsers just use https for untrusted URLs, so if you want to really log IPs you'll need to have a HTTPS website. You can do this however you want, but I reccommend you to use certbot + nginx, the instructions are fairly simple.

Even tho they reccommend to install it through `snap`, we all know that we aren't going to do so on an RPi, I personally use the [arch package](https://archlinux.org/packages/extra/any/certbot/) (also in ARM repos), but install it how you prefer.

I know there's a nginx plugin for this, but I'm dumb and I do this my own way from scratch so I ended up having my final certificates at `/etc/letsencrypt/live/myhost.net/` by running `certbot certonly --standalone`, but if you want to look into the plugin it might be better (if you do so the rest is invalid).

After having my certs I basically just:
```bash
mkdir /etc/nginx/sites-{avaliable,enabled} # Don't forget root :P
```

Added `include /etc/nginx/sites-enabled/*;` into `/etc/nginx/nginx.conf` (inside http part) and made `/etc/nginx/sites-avaliable/lipl.conf`:
```nginx
server {
    listen 80;
    server_name myhost.net;

    # Redirect HTTP to HTTPS
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name myhost.net;

    ssl_certificate /etc/letsencrypt/live/myhost.net/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/myhost.net/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
	    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```
Then just `systemctl enable --now nginx`, `crontab -e` and add `@reboot cd /home/<username>/tmp/lipl && ./lipl 8080` (I have binary under `~/tmp/lipl/lipl` because I wanna make it a service or separate it from my user). But you can do it your own way.

(I learned all this in 30 min thanks to chatGPT)
