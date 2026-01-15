# n9n

1. This section is going to be your study for n9n things

# Sources

| Label                        | URL                                                         | Notes                                                                   |
| ---------------------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------- |
| Master 80% of n8n in 60 mins | [YouTube](https://youtu.be/e3OV3LnrS7o?si=DHrrBLkKen7aFCk3) | Download and keep in **desktop** reference folder.                      |
| n8n GitHub                   | [n8n-io/n8n](https://github.com/n8n-io/n8n)                 | Fair-code, source-available repo; separate paid cloud offering. [web:3] |

# Notes

# Tool Installs

1. This will be related tools that were installed

|                         tul                         |                  wat                  |
| :-------------------------------------------------: | :-----------------------------------: |
|  [`snitch`](https://github.com/karol-broda/snitch)  | Better `ss` and `netstat` alternative |
| [`cargo-vet`](https://github.com/mozilla/cargo-vet) |     cargo suppply chain security      |

# Install

1. Installation is via the command

```js
npx n8n
```

- This is using npx , you will use bun instead

```js
bunx n8n
```

- note about 1789 deps , all had to be resolved
- It also uses oracleDB for some reason

## Docker Method

1. Based on the docs you can set it up like this , in bash

```sh
docker volume create n8n_data
docker run -it --rm --name n8n -p 5678:5678 -v n8n_data:/home/node/.n8n docker.n8n.io/n8nio/n8n
```

- The above was converted to a docker compose file which is more configurable

```yaml
version: "3.8"

services:
  n8n:
    image: docker.n8n.io/n8nio/n8n
    container_name: n8n
    ports:
      - "5678:5678"
    volumes:
      - n8n_data:/home/node/.n8n
    restart: unless-stopped

volumes:
  n8n_data:
```

- Intiial execution ( docker compose up)
- AFter shutting down , do the same command and it will be back up

> [!WARNING]
> Docker in docker is causing some issues in codespaces, wrt to the origin headers, Dont use docker method.

# Passwords

1. When starting up it you have to set pwd
2. u - panty@smell.com / p - Lickpussy9 / Email(fama) - `zikora.gurtaj@flyovertrees.com`
3. Note it required a license key - `6e65c5f0-0396-47c9-9df4-291802e1685f`
4. Note when you shut down , by `CTRl+C` , it is already installed and you can bring it back up with `bunx n8n`, this time it doesnt ask for license key.
5. BootyFart - `sk-or-v1-85` , for some reason on the restart had to make another key
6. Confirmed bunx method works
