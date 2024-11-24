# TODO

## Deployments

### Deployment Stack
- OpenStack: bare metal provisioning
- Terraform: virtual machine cloud provisioning
- Orchestration: Kubernetes pods
- Services: Docker containers
- Binaries: per OS packages and repositories
- Source code: Gitea / Forgejo
- PKI

### Docker

Add Dockerfile and docker-compose.yaml.

### Helm / Kubernetes

Add helm chart.

### Packages and Binaries
- Flatpack
- AppImage
- Ubuntu Snap
- Nix

### Architecture
- OS
  - Linux
    - Ubuntu / Debian 
    - Alpine
    - OpenSUSE
    - Fedora / Rocky
    - Arch
    - Slackware
    - Gentoo
  - BSD
    - FreeBSD
    - OpenBSD
    - NetBSD
  - Mac OS X
  - Windows
- Arch
  - amd64
  - arm v6, v7, v8
  - risc v

### Encryption / PKI
- Sign all assets
  - sigstore
  - minisign [ ed25519 ]
  - GPG [ ed25519 ]
- Checksums
  - BLAKE-3
