#
* Add `portmappings` to `ContainerCreateOptsBuilder` [#117](https://github.com/vv9k/podman-api-rs/pull/117)
* Fix typo `referene` in `PullOptsBuilder` [#119](https://github.com/vv9k/podman-api-rs/pull/119)
* Make Images::pull return a stream [#121](https://github.com/vv9k/podman-api-rs/pull/121)
* *BREAKING* All API structs no longer have a `'podman` lifetime. This change makes it easier to create self working objects without
  the lifetime hell and according to hyper client documentation it is cheap to clone and cloning is the recommended way to share a client.
* Add `Container::copy_to`, `Container::copy_file_into` and `Container::copy_from` methods to manipulate file transfers to and from the container.


# 0.2.3 
* Fix `Images::prune` return type.

# 0.2.2
* Fix `port_bindings` field of `InspectContainerHostConfig` and `InspectPodInfraConfig`. The value in the hashmap can be null.
* Fix `PortMap` type signature.

# 0.2.1
* Fix `Container::checkpoint` endpoint and doc example
* Fix return type of `ImagePushOptsBuilder::build` to `ImagePushOpts`
* Fix `ImagePushOptsBuilder::auth` method to take builder by value rather than by mutable reference
* Fix `Pod::top_stream`, `Pod::stats` - it shouldn't be async
* Fix `Pod::stats` - it shouldn't be async

# 0.2.0
* Add `Container::attach` method
* Add `Container::changes` method
* Add `Container::logs` method
* Add `Container::stats`, `Container::stats_stream`, `Containers::stats` and `Containers::stats_stream` methods
* Add `Container::top` and `Container::top_stream` methods
* Add `Containers::list_mounted`
* Add `Image::changes`
* Add `Image::tree`
* Add `Images::remove`
* Add `generate_systemd_units` method to `Pod` and `Container`
* Add `generate_kube_yaml` method to `Pod` and `Container`
* Add `Podman::play_kubernetes_yaml`
* Add `Podman::remove_kubernetes_pods`
* Add `Manifests::create`
* Add `Manifest::inspect`
* Add `Manifest::add_image`
* Add `Manifest::remove_image`
* Add `Manifest::push`
* Add `Network::delete` and `Network::force_delete`
* Add `Networks::create`
* Add `Network::inspect`
* Add `Networks::list`
* Add `Networks::prune`
* Add `Network::disconnect_container` and `Container::disconnect`
* Add `Network::connect_container` and `Container::connect`
* Fix build on Windows
* Add `Image::push`
* Add `Images::prune`
* Add `Images::search`
* Add `Images::export`
* Add `Containers::prune`
* Add `Container::healtcheck`
* Fix return value of `Podman::info`
