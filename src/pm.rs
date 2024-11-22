// pm.rs
//
// package manager struct

use crate::package::Package;
use crate::resolve::{resolve_deps, find_dependants, deep_dependants};
use crate::{die, vpr, pr, msg, erm};
use crate::tracking::{self, cache_changes};
use crate::utils::{do_install, display_list};
use crate::core::{confirm_removal, mint, download, fetch, prune_sources};
use crate::flags::FORCE;
use indicatif::{ProgressStyle, ProgressBar};

pub struct PM {
    pub pkgs: Vec<Package>,
    pub pkglist: Vec<Package>,
}

impl PM {
    pub fn new(pkgs: Vec<Package>, pkglist: Vec<Package>) -> Self {
        PM { pkgs, pkglist }
    }

    pub fn list(&self) {

        let displayed = if self.pkgs.is_empty() {
            self.pkglist.clone()
        } else {
            self.pkgs
                .iter()
                .filter_map(|pkg| self.pkglist.iter().find(|p| p.name == *pkg.name).cloned())
                .collect()
        };

        if displayed.len() != self.pkgs.len() {
            if let Some(pkg) = self.pkgs
                .iter()
                .find(|pkg| !displayed.iter().any(|p| p.name == *pkg.name))
            {
                die!("Package '{}' missing from pkglist", pkg)
            }
        }

        msg!("PACKAGES");
        display_list(displayed);
    }

    pub fn cache(&mut self) {
        msg!("Caching packages to json...");
        match cache_changes(&mut self.pkglist, true) {
            Ok(n) => msg!("Cached {} packages", n),
            Err(e) => erm!("Failed to cache: {}", e)
        }
    }

    pub fn dependencies(&self) {
        for pkg in self.pkgs.iter() {
            let d = resolve_deps(pkg, self.pkglist.clone());
            msg!("Dependencies for {}", pkg);
            display_list(d);
        }
    }

    pub fn get(&self) {
        for pkg in self.pkgs.clone() {
            msg!("Getting files for {}", pkg);
            download(pkg, true);
        }
    }

    pub fn install(&mut self) {
        for pkg in self.pkgs.iter() {
            if do_install(pkg) {
                fetch(pkg);
                mint('i', pkg);
                tracking::add(&mut self.pkglist, pkg);
                msg!("Installed '{}'", pkg);
            }
        }
    }

    pub fn update(&mut self) {
        for pkg in self.pkgs.iter() {
            if pkg.installed_version == pkg.version && !*FORCE.lock().unwrap() {
                msg!("Package '{}' up to date", pkg);
                return;
            }

            msg!("Updating '{}'...", pkg);
            fetch(pkg);
            mint('u', pkg);

            tracking::add(&mut self.pkglist, pkg);
            msg!("Updated '{}'", pkg);
        }
    }

    pub fn remove(&mut self) {
        for pkg in self.pkgs.iter() {
            if !confirm_removal(pkg, &self.pkglist) {
                return
            }

            mint('r', pkg);
            tracking::rem(&mut self.pkglist, pkg);
            msg!("Removed '{}'", pkg);
        }
    }

    pub fn news(&mut self) {
        for pkg in self.pkgs.iter() {
            if !pkg.news.is_empty() {
                msg!("News for '{}':", pkg);
                pr!("\x1b[31;3m{}\x1b[0m\n", pkg.news);
            }
        }
    }

    pub fn prune(&mut self) {
        const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {pos}/{len} ({eta})";
        let mut tarballs_removed = 0;
        let length = self.pkgs.len() as u64;
        let bar = ProgressBar::new(length);

        bar.set_message("Pruning packages");
        bar.set_style(
            ProgressStyle::with_template(BAR)
                .unwrap()
                .progress_chars("#|-"),
        );
        bar.set_length(length);

        for pkg in self.pkgs.iter() {
            vpr!("Pruning {}", pkg);
            let num_removed = prune_sources(pkg);
            vpr!("Pruned {} tarballs for '{}'", num_removed, pkg);
            tarballs_removed += num_removed;
            bar.inc(1);
        }

        bar.finish_with_message("Pruned");
        msg!("Pruned {} tarballs for {} packages", tarballs_removed, length);
    }
}
