use std::ffi::{OsStr, OsString};
use std::fmt::{self, Debug};
use std::io::Result;
use std::ptr;

use output::Output;

pub struct Winp {
    program: OsString,
    args: OsString,
}

impl Winp {
    pub fn new<S>(program: S) -> Self
    where S: AsRef<OsStr> {
        let program = {
            let mut p = OsString::from("\"");
            p.push(program.as_ref());
            p.push("\"");
            p
        };
        Winp {
            program: program,
            args: OsString::new(),
        }
    }

    pub fn arg<S>(&mut self, arg: S) -> &mut Winp
    where S: AsRef<OsStr> {
        self.args.push(" \"");
        self.args.push(arg.as_ref());
        self.args.push("\"");
        self
    }

    pub fn args<S>(&mut self, args: &[S]) -> &mut Winp
    where S: AsRef<OsStr> {
        for i in args {
            self.arg(i);
        }
        self
    }

    pub fn output(&mut self) -> Result<Output> {
        use std::os::windows::ffi::OsStrExt;
        use std::slice;
        use exit_status::ExitStatus;
        use winp_sys as sys;

        let mut winp = sys::winp_t::new();
        let cmd: Vec<_> = {
            let mut c = self.program.clone();
            c.push(&self.args);
            c.encode_wide()
                .chain(Some(0).into_iter())
                .collect()
        };
        let ret = unsafe {
            sys::winp_run_w(&mut winp, cmd.as_ptr(), ptr::null(), 0)
        };

        let status = ExitStatus::new(ret == 1, winp.return_code as i32);
        let stdout = if winp.output.is_null() {
            Vec::new()
        } else {
            unsafe {
                slice::from_raw_parts(winp.output as *const _, winp.output_len)
                    .to_vec()
            }
        };
        let stderr = if winp.error.is_null() {
            Vec::new()
        } else {
            unsafe {
                slice::from_raw_parts(winp.error as *const _, winp.error_len)
                    .to_vec()
            }
        };
        let output = Output {
            status: status,
            stdout: stdout,
            stderr: stderr,
        };

        unsafe {
            sys::winp_free(&mut winp);
        }

        Ok(output)
    }
}

impl Debug for Winp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{}{}",
            self.program.to_string_lossy(), self.args.to_string_lossy())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::Winp;

    #[test]
    fn cat_help_command_test() {
        let output = Winp::new("cat").arg("--help").output().unwrap();
        assert_eq!(output.status.success(), true);
        assert_eq!(output.status.code().unwrap(), 0);
        assert!(output.stdout != Vec::new());
        assert!(output.stderr == Vec::new());
    }

    #[test]
    fn unknown_command_test() {
        let output = Winp::new("unknown").output().unwrap();
        assert_eq!(output.status.success(), false);
        assert_eq!(output.status.code().unwrap(), 0);
        assert!(output.stdout == Vec::new());
        assert!(output.stderr == Vec::new());
    }

    #[test]
    fn echo_command_test() {
        let output = Winp::new("echo")
            .args(&["arg1", "arg 2", " arg3"])
            .output()
            .unwrap();
        assert_eq!(output.status.success(), true);
        assert_eq!(output.status.code().unwrap(), 0);
        assert!(output.stdout == b"arg1 arg 2  arg3\n");
        assert!(output.stderr == Vec::new());
    }

    #[test]
    fn fn_debug() {
        let mut w = Vec::new();
        write!(&mut w, "{:?}", Winp::new("echo")).unwrap();
        assert_eq!(w, b"\"echo\"");

        w.clear();
        write!(&mut w, "{:?}", Winp::new("echo").arg("foo bar")).unwrap();
        assert_eq!(w, b"\"echo\" \"foo bar\"");
    }
}
