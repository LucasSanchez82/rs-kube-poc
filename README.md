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

key.pem :

```sh
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAqcpRZ4OhPOywPqKTs7Uv/2U5eL85UXsyirbZma6X6XEr3w9/
POArWR5oqTdkR6eTcBLB5TijqcT6CbXqAopcrLoJbup7yLFjThQhwJGyFezMlxjZ
8Wz8osfFheyc11y9tRWaMiwXO7dRdnBcUP/O6fM2JuifAHrLYTiwWGKKThRgrsDo
MU+nta2y1lvB4e8Bf73y6IkYgnB5CChSxASSIWg4XZJEZyg1HA3V6jhzpCMIlR7F
ZjzFH0vX+mZsXjmYdMzlHJJNHHw6NdneP2kPOgZozP9IcWG8BV7vbwXR1efSCmo5
qsRkAmEw6o3yHzr/p09Xu2REh4B2ssi/8MNS9QIDAQABAoIBAC93GLS4bo3KmSaZ
anMVltpt4NgplQt7RlDS9xfoYB1pfts3yNkqzdN3FckgbA9AAI6XH89FuzMNlty6
FzYomenLhDGWqY4cUOeV5y2fTUqlzTf8QYklb6bN9CERWXM3QO1roZSfEMun3sUe
eiBGswBh9mSAKVsH+O0v3NSIj6NtyFQs2r0GMZDIxXkke5gKC9xOYlMRFVqGjqTa
8mABWrG2ruXbEUU5Zc4w3MS0WUO6w7xKqDIN9iITjcp61I9bj/5keEXAnV1jM+Gp
m2AK2BvN0gWQKqMxUOvbctPslg2gLCZ2sdFko9ubaFfxwvy4Obtbm43ym9XcWb/E
kc42/c0CgYEA1We0Z3XNvQvO4+R+Insy1QVt8XxDLQB82VvJc24HvsFdVRaoLZ/A
rrDsb7RM9glnF0Tyl78y7YTPjFJw6DxHnx1Scfrkq2cxG8kPNMI9kiHoEmOPQSrE
hZJLBUh3nepgnWz+E9HaTqMFTHk+qsY7ijtZWlrVMIHfP/lrQsXGalcCgYEAy64K
obXkKrPJV5ovfo6wYcQZyAeFltYj9D8QtDrQVvjm+4bT2yp2wVk4v6+fluwnlZt7
VCqM666jlu2pbrIBaMb9mvcwFRHNDOQESU2vujQGDgpHUnlEW0evtFXSCs7zO/Sz
QLA0XFL5oniS/jS/tRa055JM9fkw7pDPnNkJ9ZMCgYBQ9PyNWzVniDk+XwoyzoXk
JiElQTzCr7KtSpLLxWwOw5BCnUvDsx0HqHqpjb68f6iqPS/CvXf2fzG1S/klQOjy
xc7nCoUBOS56GfY4z49IBrDheP95e13+RhdLs8gdEjMAxb5FipDE0DdMYZqSco1X
zVgCXP8PXdVxkY9NT3YMFQKBgQDAP8eTOZ9i2w4pnMRFnaNItJNtJgRnCjsnmLtI
ktcBsLA9K9ceYGd1OqUA8WEeUEtQwFBZckxPTSjmFe++J1CDOIg51CXTTVkRWMve
8O/0PpSUfgkyBHqL+g8TlYnLX+lgZykm3uqdVkiAnXSJcvqckBEzHvl7lrkkBRWh
w47N9wKBgQCxEBnaFKINwFlQ4paNFAOoshuPVFNjtaUilgKZUEG0PK+6Lk2yu84+
9aSFoKEUgKt5D8KqHrKu62qqs9qnsLy34ghfkpXXjus7X1/X8UsSZ+M/tnNdhrIy
gjL4GOcv5WgZYsHng2/8o/QMAUfyKipu36BMYSxUlT96fakQi9y1ng==
-----END RSA PRIVATE KEY-----
```

## Run

```
cargo run
```

now you can import `bruno` collections ( postman equivalent ) to test requests
