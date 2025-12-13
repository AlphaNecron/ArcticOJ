static TEST_PROG: &str = include_str!("test_progs/main.py");

#[derive(Clone)]
enum Type {
    // PyPy2,
    PyPy3,
    // Py2,
    Py3,
}

fn self_test() {}

// #[executor(bin = "python3", self_test = self_test, test_prog = TEST_PROG, ty = "Compiled")]
struct Python3;

// #[executor(bin = "pypy3", self_test = self_test, test_prog = TEST_PROG, ty = "Compiled")]
struct PyPy3;

// fn new(ty: Type, bin: &str) -> Executor {
//     Executor::new(
//         match ty {
//             // Type::PyPy2 => "pypy2",
//             Type::PyPy3 => "pypy3",
//             // Type::Py2 => "python2",
//             Type::Py3 => "python3",
//         }
//         .to_string(),
//         ExecutorType::Compiled,
//         bin,
//         PyEx(ty),
//         // include_str!("test_progs/main.py")
//     )
// }
//
// pub(super) fn python3() -> Executor {
//     new(Type::Py3, "python3")
// }
//
// pub(super) fn pypy3() -> Executor {
//     new(Type::PyPy3, "pypy3")
// }
