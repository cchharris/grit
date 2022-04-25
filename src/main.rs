#![allow(dead_code)]

use std::env;

mod string_list;
mod color;

const GIT_USAGE_STRING: &str = concat!(
    "grit [--version] [--help] [-C <path>] [-c <name>=<value>]\n",
    "           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]\n",
    "           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]\n",
    "           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]\n",
    "           [--super-prefix=<path>] [--config-env=<name>=<envvar>]\n",
    "           <command> [<args>]"
);

const GIT_MORE_INFO_STRING: &str = concat!(
    "'grit help -a' and 'git help -g' list available subcommands and some\n",
    "concept guides. See 'grit help <command>' or 'grit help <concept>'\n",
    "to read about a specific subcommand or concept.\n",
    "See 'grit help grit' for an overview of the system."
);

// In git this is a bitfield.  However, this seems to only be used internally to track
// options to cmds.  Let's not overcomplicate things and just use booleans for now.
struct BuiltinOptions {
    run_setup: bool,
    run_setup_gently: bool,
    use_pager: bool,
    need_work_tree: bool,
    support_super_prefix: bool,
    delay_pager_config: bool,
    no_parse_opt: bool,
}

struct Command {
    cmd: &'static str,
    fun_ptr: &'static dyn Fn(Vec<&str>, &str),
    options: BuiltinOptions,
}

fn main() {
    let args: Vec<String> = env::args().collect();
}
