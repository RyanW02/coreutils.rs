use std::env;

// required for null terminated strings
use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn puts(s: *const c_char);
}

#[derive(Debug)]
struct Options {
    multiple: bool,
    remove_suffix: Option<String>,
    zero: bool,
    paths: Vec<String>,
}

fn main() {
    let options = parse_options(env::args());

    options.paths.iter().map(|s| &s[..]).for_each(|path| {
        let mut path = match path.strip_suffix("/") {
            Some(stripped) => stripped,
            None => path,
        };

        if let Some(split_index) = path.rfind("/") {
            let (_, right) = path.split_at(split_index + 1);
            path = right;
        }

        if let Some(suffix) = &options.remove_suffix {
            if let Some(stripped) = path.strip_suffix(&suffix[..]) {
                path = stripped;
            }
        }

        if options.zero {
            let null_terminated = CString::new(path).unwrap();
            unsafe {
                puts(null_terminated.as_ptr());
            }
        } else {
            println!("{}", path);
        }
    });
}

// parse flags
fn parse_options(mut args: impl Iterator<Item=String>) -> Options {
    let mut options = Options {
        multiple: false,
        remove_suffix: None,
        zero: false,
        paths: Vec::new(),
    };

    let mut is_first = true;
    let mut next_is_suffix = false;

    while let Some(arg) = args.next() {
        if is_first {
            is_first = false;
            continue;
        }

        match &arg[..] {
            "-a" => options.multiple = true,
            "-z" => options.zero = true,
            "-s" => {
                if !next_is_suffix {
                    next_is_suffix = true;
                }

                options.multiple = true;
            }
            _ => {
                if next_is_suffix || (!options.multiple && options.paths.len() > 0) {
                    next_is_suffix = false;
                    options.remove_suffix = Some(arg);
                } else {
                    options.paths.push(arg.clone());
                }
            }
        }
    }

    options
}
