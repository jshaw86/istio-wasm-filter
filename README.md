# Deps
- docker installed
- kubectl
- kubernetes
- istioctl
- dockerhub or other oci compatible repo

# Setup 
This provisions istio and sets up httpbin to your kubernetes cluster

- `curl -L https://istio.io/downloadIstio | sh -`
- `cd istio-<version>` # current version is 1.21.0 
- `export PATH=$PWD/bin:$PATH` # make istioctl available
- `istioctl install`
- `kubectl create namespace httpbin`
- `kubectl apply -f httpbin-vs-gateway.yaml -n httpbin`
- change the oci url in `wasm-plugin.yaml`

# Build
The below will build the rust lib with `wasm-pack`, create and push a tag to your docker/repo. The wasm plugin currently only tags 

- `docker build . -t <docker/repo:tag> # example dockerio_username/wasm:latest` 
- `docker push <docker/repo:tag>`
- update tag in wasm-plugin
- `kubectl apply -f wasm-plugin.yaml # deployed to istio-system targeting gateway`


