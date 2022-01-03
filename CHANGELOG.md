# 0.2.0
---

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
