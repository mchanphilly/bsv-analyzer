//! In rust-analyzer, we maintain a strict separation between pure abstract
//! semantic project model and a concrete model of a particular build system.
//!
//! Pure model is represented by the [`base_db::CrateGraph`] from another crate.
//!
//! In this crate, we are concerned with "real world" project models.
//!
//! Specifically, here we have a representation for a Cargo project
//! ([`CargoWorkspace`]) and for manually specified layout ([`ProjectJson`]).
//!
//! Roughly, the things we do here are:
//!
//! * Project discovery (where's the relevant Cargo.toml for the current dir).
//! * Custom build steps (`build.rs` code generation and compilation of
//!   procedural macros).
//! * Lowering of concrete model to a [`base_db::CrateGraph`]

mod build_dependencies;
mod cargo_workspace;
mod env;
mod manifest_path;
pub mod project_json;
mod rustc_cfg;
mod sysroot;
pub mod target_data_layout;
mod workspace;

#[cfg(test)]
mod tests;

use std::{
    fmt,
    fs::{self, read_dir, ReadDir},
    io,
    process::Command,
};

use anyhow::{bail, format_err, Context};
use itertools::Itertools;
use paths::{AbsPath, AbsPathBuf, Utf8PathBuf};
use rustc_hash::FxHashSet;

pub use crate::{
    build_dependencies::WorkspaceBuildScripts,
    cargo_workspace::{
        CargoConfig, CargoFeatures, CargoWorkspace, Package, PackageData, PackageDependency,
        RustLibSource, Target, TargetData, TargetKind,
    },
    manifest_path::ManifestPath,
    project_json::{ProjectJson, ProjectJsonData},
    sysroot::Sysroot,
    workspace::{FileLoader, PackageRoot, ProjectWorkspace, ProjectWorkspaceKind},
};
pub use cargo_metadata::Metadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectJsonFromCommand {
    /// The data describing this project, such as its dependencies.
    pub data: ProjectJsonData,
    /// The build system specific file that describes this project,
    /// such as a `my-project/BUCK` file.
    pub buildfile: AbsPathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ProjectManifest {
    ProjectJson(ManifestPath),
    CargoToml(ManifestPath),
    CargoScript(ManifestPath),
    BluespecFile(ManifestPath),
}

impl ProjectManifest {
    pub fn from_manifest_file(path: AbsPathBuf) -> anyhow::Result<ProjectManifest> {
        let path = ManifestPath::try_from(path)
            .map_err(|path| format_err!("bad manifest path: {path}"))?;

        if path.file_name().unwrap_or_default() == ".rust-project.json" {
            return Ok(ProjectManifest::ProjectJson(path));
        }

        // TODO BSV, we may instead seek a config file (e.g., a toml or json) to describe projects later
        if path.extension().unwrap_or_default() == "bsv" ||
            path.extension().unwrap_or_default() == "ms" {
            return Ok(ProjectManifest::BluespecFile(path));
        }
        bail!("project root must point to a .bsv or .ms file: {path}");
    }

    pub fn discover_single(path: &AbsPath) -> anyhow::Result<ProjectManifest> {
        let mut candidates = ProjectManifest::discover(path)?;
        let res = match candidates.pop() {
            None => bail!("no projects"),
            Some(it) => it,
        };

        if !candidates.is_empty() {
            bail!("more than one project");
        }
        Ok(res)
    }

    pub fn discover(path: &AbsPath) -> io::Result<Vec<ProjectManifest>> {
        const DEPTH: usize = 7;  // perhaps overkill
        // TODO BSV add in a way to connect things together.
        // if let Some(project_json) = find_in_parent_dirs(path, "rust-project.json") {
        //     return Ok(vec![ProjectManifest::ProjectJson(project_json)]);
        // }
        dbg!(&path);
        let target_exts = &["bsv", "ms"];

        // // Performs a traversal of the file system from the workspace root provided by LS client on launch
        // // or maybe other times too, e.g., if we add a dependency.
        return find_exts(path, target_exts)
            .map(|paths| paths.into_iter().map(ProjectManifest::BluespecFile).collect());

        fn find_exts(path: &AbsPath, target_exts: &[&str]) -> io::Result<Vec<ManifestPath>> {
            let res =
                Ok(find_exts_in_child_dir(read_dir(path)?, target_exts, DEPTH, 0));
            res
        }

        fn find_exts_in_child_dir(entities: ReadDir, target_exts: &[&str], max_depth: usize, current_depth: usize) -> Vec<ManifestPath> {
            // Now multiple layers. May appear strange in cases of weird file structure (e.g., cyclic links)
            // Populate the directories we're looking for first
            if current_depth >= max_depth {
                return Vec::new(); // Stop searching deeper if we have reached max depth
            }
    
            let mut result = Vec::new();
    
            for entry in entities.filter_map(Result::ok) {
                let path = entry.path();
                
                if let Some(ext) = path.extension() {
                    if target_exts.contains(&ext.to_str().unwrap_or("")) {
                        let candidate: Vec<_> = std::iter::once(path)
                            .map(Utf8PathBuf::from_path_buf)
                            .filter_map(Result::ok)
                            .map(AbsPathBuf::try_from)
                            .filter_map(Result::ok)
                            .map(ManifestPath::try_from)
                            .filter_map(Result::ok)
                            .collect();
                        result.extend(candidate);
                    }
                }

                // Recurse into subdirectories, increasing the depth
                if entry.path().is_dir() {
                    if let Ok(subdir) = read_dir(entry.path()) {
                        result.extend(find_exts_in_child_dir(subdir, target_exts, max_depth, current_depth + 1));
                    }
                }
            }
    
            result
        }
    }

    pub fn discover_all(paths: &[AbsPathBuf]) -> Vec<ProjectManifest> {
        let mut res = paths
            .iter()
            .filter_map(|it| ProjectManifest::discover(it.as_ref()).ok())
            .flatten()
            .collect::<FxHashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        res.sort();
        res
    }

    pub fn manifest_path(&self) -> &ManifestPath {
        match self {
            ProjectManifest::ProjectJson(it)
            | ProjectManifest::CargoToml(it)
            | ProjectManifest::CargoScript(it)
            | ProjectManifest::BluespecFile(it) => it,
        }
    }
}

impl fmt::Display for ProjectManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.manifest_path(), f)
    }
}

fn utf8_stdout(mut cmd: Command) -> anyhow::Result<String> {
    let output = cmd.output().with_context(|| format!("{cmd:?} failed"))?;
    if !output.status.success() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) if !stderr.is_empty() => {
                bail!("{:?} failed, {}\nstderr:\n{}", cmd, output.status, stderr)
            }
            _ => bail!("{:?} failed, {}", cmd, output.status),
        }
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.trim().to_owned())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum InvocationStrategy {
    Once,
    #[default]
    PerWorkspace,
}

/// A set of cfg-overrides per crate.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct CfgOverrides {
    /// A global set of overrides matching all crates.
    pub global: cfg::CfgDiff,
    /// A set of overrides matching specific crates.
    pub selective: rustc_hash::FxHashMap<String, cfg::CfgDiff>,
}

impl CfgOverrides {
    pub fn len(&self) -> usize {
        self.global.len() + self.selective.values().map(|it| it.len()).sum::<usize>()
    }

    pub fn apply(&self, cfg_options: &mut cfg::CfgOptions, name: &str) {
        if !self.global.is_empty() {
            cfg_options.apply_diff(self.global.clone());
        };
        if let Some(diff) = self.selective.get(name) {
            cfg_options.apply_diff(diff.clone());
        };
    }
}

fn parse_cfg(s: &str) -> Result<cfg::CfgAtom, String> {
    let res = match s.split_once('=') {
        Some((key, value)) => {
            if !(value.starts_with('"') && value.ends_with('"')) {
                return Err(format!("Invalid cfg ({s:?}), value should be in quotes"));
            }
            let key = intern::Symbol::intern(key);
            let value = intern::Symbol::intern(&value[1..value.len() - 1]);
            cfg::CfgAtom::KeyValue { key, value }
        }
        None => cfg::CfgAtom::Flag(intern::Symbol::intern(s)),
    };
    Ok(res)
}
