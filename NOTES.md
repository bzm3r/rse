# TODO

## Important
* Create a NixOS module for actually adding in RSE.
* Allow registry of "known manifests", which the user can simply instantiate by name rather than having to always write out a complete `cargo` manifest within a single-page rust script.

## Stretch Goals
* allow passing in an additional "known_scripts" registry, to non-declaratively (as far as NixOS configuration is concerned) expand the scripts known to RSE?
* support input script from `stdin`?
