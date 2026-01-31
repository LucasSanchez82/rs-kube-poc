# Super project without name

## Description

tree

```bash
.
├── Cargo.lock
├── Cargo.toml
├── config.sh
├── connect.sh
├── forward.sh
├── install_minikube.sh
├── README.md
├── key.pem             <-- should be added
├── Rocket.toml
└── src
    ├── controllers
    │   ├── database.rs
    │   └── mod.rs
    ├── main.rs
    ├── structs
    │   ├── database_provider.rs
    │   ├── mod.rs
    │   └── responder.rs
    └── utils
        ├── log.rs
        └── mod.rs
```

## Prerequisites

- Launch an EC2 instance using aws
- Create a new `key.pem` that correspond to the private key of the ec2 instance
- Copy and paste bash script from `install_minikube.sh` like `./install_minikube.sh <ip>`
- Run `config.sh` like `./config.sh <ip>` to get certificates from minikubes ( warning: look at the script, if you don't want to loss you actual kubeconfig )
- Run `forward.sh` like `./forward.sh <ip>` to be connected in ssh and to share / forward the port 8773 -> 32771 with the remote minikube

```
cargo run
```

now you can import `bruno` collections ( postman equivalent ) to test requests
