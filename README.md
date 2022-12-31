# recast_navigation
Rust wrappers for recast navigation

[![Cargo](https://img.shields.io/crates/v/crossbeam.svg)](
https://crates.io/crates/recast_navigation)
[![Documentation](https://docs.rs/recast_navigation/badge.svg)](
https://docs.rs/recast_navigation)

This is a wrapper over a higher level C++ wrapper for recast navigation.  So the rust side is fairly high level.
Supports recast, detour, and detour crowd to varying degrees.  Some functionality like offmesh links are not
supported.  

## Building from source
The C++ source is built with cmake and has VS cmake project configurations.
Precompiled libraries for windows and linux are provided in lib. build.rs will copy them to the build output.

If you build from source you will want to copy the shared libs cmake creates into lib manually.

## Navmesh building flow

* Create a NavmeshBuilder.
* Create a Navmesh.
* NavmeshBuildUtils::get_overlapping_tiles_from_settings will give you a list of all tile coords within a bounds.
* Take those tiles and call NavmeshBuildUtils::calculate_tile_bounding_box on each tile to get the tile bounds.
* Create a TileInputBuilder for each tile.
* Gather the geometry for the tile bounds and feed it to the TileInputBuilder.
* NavmeshBuilder has a build_tile function that takes a TileInputBuilder. Single builder can build multiple tiles.
* Add the tiles to the navmesh

Tile building is completely separate from the navmesh. So you can build tiles in the background at runtime,
and then add/replace them into the navmesh which is fairly fast/cheap.

## Navmesh Queries
Queries are done through a NavQuery instance.  Queries have a pointer to the Navmesh, so cannot be used while adding/removing tiles from the navmesh.

We ensure this by queries being obtained from a query pool belonging to the navmesh, and some simple reference counting.
If there are rented queries you won't be allowed to mutate the navmesh.

## moving agents over the navmesh
The flow here is straight forward but it's important to understand that there is a navigation mesh and a separate detail mesh.

You pathfind over the navigation mesh, but it does not return a correct surface height.
You call NavQuery.get_location with a position that is on the path, which returns the surface height.


## Crowds
Detour crowd is decent but it's the one area of recast that people usually customize or just roll their own.
Don't expect it's behavior to match what engines like UE or Unity provide, as they have heavily customized detour crowd over time.