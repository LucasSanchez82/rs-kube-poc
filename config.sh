#!/bin/bash

# Configuration
REMOTE_USER="admin"
REMOTE_HOST=$1  # ou l'IP publique
key="key.pem"
LOCAL_KUBE_DIR="$HOME/.kube/minikube-remote"

# Créer le dossier local
mkdir -p "$LOCAL_KUBE_DIR"

# Copier les certificats depuis la VM (via sudo car dans /root)
sudo ssh "$REMOTE_USER@$REMOTE_HOST" -i "$key" "sudo cat /root/.minikube/ca.crt" > "$LOCAL_KUBE_DIR/ca.crt"
sudo ssh "$REMOTE_USER@$REMOTE_HOST" -i "$key" "sudo cat /root/.minikube/profiles/minikube/client.crt" > "$LOCAL_KUBE_DIR/client.crt"
sudo ssh "$REMOTE_USER@$REMOTE_HOST" -i "$key" "sudo openssl pkcs8 -topk8 -nocrypt -in /root/.minikube/profiles/minikube/client.key -out /root/.minikube/profiles/minikube/client-pkcs8.key"
sudo ssh "$REMOTE_USER@$REMOTE_HOST" -i "$key" "sudo cat /root/.minikube/profiles/minikube/client-pkcs8.key" > "$LOCAL_KUBE_DIR/client.key"


# Sur la VM

# sudp ssh "$REMOTE_USER@$REMOTE_HOST" -i "$key" "sudo cat /root/.minikube/profiles/minikube/client.key" > "$LOCAL_KUBE_DIR/client.key"

# Configurer kubectl localement (utilise localhost car tu fais du port-forward)
kubectl config set-cluster minikube-remote \
    --server=https://localhost:8443 \
    --certificate-authority="$LOCAL_KUBE_DIR/ca.crt"

kubectl config set-credentials minikube-remote \
    --client-certificate="$LOCAL_KUBE_DIR/client.crt" \
    --client-key="$LOCAL_KUBE_DIR/client.key"

kubectl config set-context minikube-remote \
    --cluster=minikube-remote \
    --user=minikube-remote

kubectl config use-context minikube-remote

echo "Done! Test avec: kubectl get nodes"