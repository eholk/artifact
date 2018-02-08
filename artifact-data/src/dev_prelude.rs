/*
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */
pub(crate) use ergo::*;
#[allow(unused_imports)]
pub(crate) use expect_macro::*;
// TODO: move these to std_prelude
pub(crate) use std::ffi::OsStr;
pub use std::cmp::Ord;
pub use std::cmp::PartialOrd;
pub use std::hash::{Hash, Hasher};
use std::io;
use std::fs;

pub(crate) use ordermap::{OrderMap, OrderSet};

pub(crate) use std::result;
pub(crate) use failure::Error;

pub(crate) type Result<V> = result::Result<V, Error>;

/// Inplace trim is annoyingly not in the stdlib
pub(crate) fn string_trim_right(s: &mut String) {
    let end = s.trim_right().len();
    s.truncate(end);
}

#[allow(dead_code)]
/// A simple implementation of "touch"
pub(crate) fn touch<P: AsRef<Path>>(path: P) -> ::std::io::Result<()> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path.as_ref())?;
    Ok(())
}

#[test]
fn sanity_trim_right() {
    let mut result = "  hello    ".into();
    string_trim_right(&mut result);
    assert_eq!(result, "  hello");
}

fn create_dir_maybe<P: AsRef<Path>>(path: P) -> path_abs::Result<PathDir> {
    let arc = PathArc::new(path);
    fs::create_dir(&arc).map_err(|err| {
        let out: result::Result<(), path_abs::Error> =
            Err(path_abs::Error::new(err, "creating dir", arc.clone()));
        return out;
    });
    PathDir::new(arc)
}

/// Copy a directory from one location to another quickly.
pub fn copy_dir<P: AsRef<Path>>(from: PathDir, to: P) -> result::Result<PathDir, Vec<io::Error>> {
    let recv_err = {
        let (send_err, recv_err) = ch::bounded(128);
        let handle_err = spawn(move || {
            recv_err.iter().collect::<Vec<io::Error>>();
        });

        let to = match create_dir_maybe(to) {
            Ok(d) => d,
            Err(err) => return Err(vec![err.into()]),
        };

        // let (send_file, recv_file) = ch::bounded(128);
        take!(send_err as errs);
        spawn(move || {
            // Do a contents-first yeild and follow any symlinks -- we are doing an _actual_ copy
            for entry in from.walk().follow_links(true).contents_first(true) {
                let entry = match entry {
                    Ok(e) => e,
                    Err(err) => {
                        ch!(errs <- err.into());
                        continue;
                    }
                };
                let postfix = expect!(
                    entry.path().strip_prefix(&from),
                    "{} does not have prefix {}",
                    entry.path().display(),
                    from.display()
                );
            }
        })
    };

    unimplemented!();
}
