#
* Deserialize Vec and HashMap to Option in case a value is null and unwrap a default if it is missing
* Add `PodCreateOptsBuilder::dns_server`
* Add `PodCreateOptsBuilder::image_volumes`
* Add `PodCreateOptsBuilder::mounts`
* Add `PodCreateOptsBuilder::overlay_volumes`
* Add `PodCreateOptsBuilder::portmappings`
* Add `PodCreateOptsBuilder::security_opt`
* Add `PodCreateOptsBuilder::service_container_id`
* Add `PodCreateOptsBuilder::share_parent`
* Add `PodCreateOptsBuilder::shm_size`
* Add `PodCreateOptsBuilder::utsns`
* Add `PodCreateOptsBuilder::volumes`
* Add `PodCreateOptsBuilder::idmappings`
* Add `PodCreateOptsBuilder::exit_policy`
* Add `ContainerCreateOptsBuilder::chroot_directories`
* Add `ContainerCreateOptsBuilder::envmerge`
* Add `ContainerCreateOptsBuilder::groups`
* Add `ContainerCreateOptsBuilder::health_check_on_failure_action`
* Add `ContainerCreateOptsBuilder::hostusers`
* Add `ContainerCreateOptsBuilder::image_os`
* Add `ContainerCreateOptsBuilder::image_variant`
* Add `ContainerCreateOptsBuilder::image_arch`
* Add `ContainerCreateOptsBuilder::manage_password`
* Add `ContainerCreateOptsBuilder::mounts`
* Add `ContainerCreateOptsBuilder::passwd_entry`
* Add `ImageBuildOptsBuilder::all_platforms`
* Add `ImageBuildOptsBuilder::unset_env`
* Add `ManifestCreateOptsBuilder::amend`
* Add `ManifestImageAddOptsBuilder::os_features`
* Add `ManifestPushOptsBuilder::quiet`
* Add `ManifestPushOptsBuilder::tls_verify`

# 0.7.0
* Make futures returned by methods Send + Sync

# 0.6.0
* Add `Container::restore`, `Container::resize`, `Container::export`
* Add support for `!=` filters like `label!=key=val` or `label!=key`
* Add missing filter function `NetworkPruneOptsBuilder::filters`
* Fix `Pods::stats` endpoint
* Fix all `exists` endpoints like `Container::exists`
* Fixed a typo that prevented the specification of a dockerfile when building an image. [#133](https://github.com/vv9k/podman-api-rs/pull/133)
* *BREAKING* Make the result stream of `Images::build` continuous [#134](https://github.com/vv9k/podman-api-rs/pull/134)
* Fix `Container::kill`
* Fix API documentation hyperlinks
* Fix `Container::mount`, `Container::unmount`
* *BREAKING* `export` option is no longer available on `ContainerCheckpointOptsBuilder`. In place of the option, there is a new `Container::checkpoint_export` method that exports the tarball as a stream while the `Container::checkpoint` method creates a snapshot on the podman side without exporting it. 
* *BREAKING* Change stream item of `Container::logs` to `TtyChunk`
* *BREAKING* Fix typo in Container method name `healtcheck` -> `healthcheck` and fix the endpoint itself.
* Fix return type of `Containers::prune`
* `Transport` is no longer publicly available in `conn` module
* Fix `Image::history` return type
* Fix `Image::untag` endpoint
* Fix `Image::tree` return type
* Fix `Images::export` to correctly return a stream of chunks
* Fix `Secret::delete` endpoint
* Fix `Manifest::remove_image` return type
* Add `Manifest::delete` method
* Fix `Manifest::create` endpoint to correctly include name of the manifest
* Replace ManifestCreateOptsBuilder `image` method with `images`
* Rename `Network::force_delete` to `Network::remove` to match other structs
* Fix `Network::delete` and `Network::remove` return value

# 0.5.0
* Fix some generated models

# 0.4.1
* Missing attributes for container creation added. [#127](https://github.com/vv9k/podman-api-rs/pull/127)
* Use correct endpoint in Pods::create  [#128](https://github.com/vv9k/podman-api-rs/pull/128)

# 0.4.0
* Use libpod API v4.2
* Update return type of `Network::inspect` and `Network::create` to `models::Network`
* Update return type of `Networks::list` to `Vec<models::Network>`
* Use simplified generic parameters wherever possible

# 0.3.0
* Add `portmappings` to `ContainerCreateOptsBuilder` [#117](https://github.com/vv9k/podman-api-rs/pull/117)
* Fix typo `referene` in `PullOptsBuilder` [#119](https://github.com/vv9k/podman-api-rs/pull/119)
* Make Images::pull return a stream [#121](https://github.com/vv9k/podman-api-rs/pull/121)
* *BREAKING* All API structs no longer have a `'podman` lifetime. This change makes it easier to create self working objects without
  the lifetime hell and according to hyper client documentation it is cheap to clone and cloning is the recommended way to share a client.
* Add `Container::copy_to`, `Container::copy_file_into` and `Container::copy_from` methods to manipulate file transfers to and from the container.
* Actually implement `Images::build` endpoint
* Fix `InspectNetworkSettings::ports` type signature

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
