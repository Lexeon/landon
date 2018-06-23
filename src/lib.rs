//! Blender files can have meshes such as circles, cubes, cylinders, a dragon or any other
//! 3D shape.
//!
//! A mesh can be represented as a group of vertices and data about those vertices, such as their
//! normals or UV coordinates.
//!
//! Meshes can also have metadata, such as the name of it's parent armature (useful for vertex
//! skinning).
//!
//! blender-mesh-to-json seeks to be a well tested, well documented exporter for blender mesh
//! metadata.
//!
//! You can write data to stdout or to a file. At the onset it will be geared towards @chinedufn's
//! needs - but if you have needs that aren't met feel very free to open an issue.
//!
//! @see https://docs.blender.org/manual/en/dev/modeling/meshes/introduction.html - Mesh Introduction
//! @see https://github.com/chinedufn/blender-actions-to-json - Exporting blender armatures / actions

#[macro_use]
extern crate failure;

use std::collections::HashMap;

/// Something went wrong in the Blender child process that was trying to parse your mesh data.
#[derive(Debug, Fail)]
pub enum BlenderError {
    /// Errors in Blender are written to stderr. We capture the stderr from the `blender` child
    /// process that we spawned when attempting to export meshes from a `.blend` file.
    #[fail(display = "There was an issue while exporting meshes: Blender stderr output: {}", _0)]
    Stderr(String),
}

/// Configuration for how to export the meshes from your `.blend` file.
#[derive(Debug)]
pub struct ExportConfig {
    /// The filepath to write the exported mesh data to.
    pub output_filepath: Option<String>
}

/// All of the data about a Blender mesh
#[derive(Debug)]
pub struct BlenderMesh {
    /// [v1x, v1y, v1z, v2x, v2y, v2z, ...]
    pub vertex_positions: Vec<f32>,
    /// The indices within vertex positions that make up each triangle in our mesh.
    /// Three vertex position indices correspond to one triangle
    /// [tri1
    pub vertex_position_indices: Vec<f32>,
    pub vertex_normals: Vec<f32>,
    pub vertex_normal_indices: Vec<f32>,
    pub vertex_uvs: Option<Vec<f32>>,
    pub vertex_uv_indices: Option<Vec<f32>>,
    pub texture_name: String,
    pub armature_name: String,
}

pub type MeshNamesToData = HashMap<String, BlenderMesh>;
pub type FilenamesToMeshes = HashMap<String, MeshNamesToData>;

/// Given a buffer of standard output from Blender we parse all of the mesh JSON that was
/// written to stdout by `blender-mesh-to-json.py`.
///
/// Meshes data in stdout will look like:
///
/// ```
/// START_MESH_JSON /path/to/file.blend my_mesh_name
/// END_MESH_JSON /path/to/file.blend my_mesh_name
/// ```
///
/// @see blender-mesh-to-json.py - This is where we write to stdout
pub fn parse_meshes_from_blender_stdout (stdout: &str, config: Option<&ExportConfig>)
    -> Result<FilenamesToMeshes, failure::Error> {
    let filenames_to_meshes = HashMap::new();
    // TODO: JSON Output format [{mesh_name: 'foo', mesh_data: {}, source_file: '/path/to/file.blend'}]

    // TODO: Breadcrumb - define the `Mesh` struct, then start writing data to stdout from Blender

    Ok(filenames_to_meshes)
}