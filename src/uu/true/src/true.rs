//  * This file is part of the uutils coreutils package.
//  *
//  * (c) Jordi Boggiano <j.boggiano@seld.be>
//  *
//  * For the full copyright and license information, please view the LICENSE
//  * file that was distributed with this source code.

#[macro_use]
extern crate uucore;

use clap::App;
use uucore::error::UResult;

#[uucore_procs::gen_uumain]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    uu_app().get_matches_from(args);
    Ok(())
}

pub fn uu_app() -> App<'static, 'static> {
    App::new(uucore::util_name())
}
