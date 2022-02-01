#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use {
    console_error_panic_hook::set_once as set_panic_hook,
    koto::{
        runtime::{KotoFile, KotoRead, KotoWrite, RuntimeError},
        Koto, KotoSettings,
    },
    std::{cell::RefCell, fmt, rc::Rc},
    web_sys::window,
};

fn main() {
    set_panic_hook();
    run_app();
}

fn run_app() {
    let document = window()
        .and_then(|w| w.document())
        .expect("Failed to access document");
    let body = document.body().expect("Failed to access document.body");

    let text = compile_and_run("print 'Hello from Koto!'");

    let text_node = document.create_text_node(&text);
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}

// Captures output from Koto in a String
#[derive(Debug)]
struct OutputCapture {
    output: Rc<RefCell<String>>,
}

impl KotoFile for OutputCapture {}
impl KotoRead for OutputCapture {}

impl KotoWrite for OutputCapture {
    fn write(&self, bytes: &[u8]) -> Result<(), RuntimeError> {
        let bytes_str = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string().into()),
        };
        self.output.borrow_mut().push_str(bytes_str);
        Ok(())
    }

    fn write_line(&self, output: &str) -> Result<(), RuntimeError> {
        let mut unlocked = self.output.borrow_mut();
        unlocked.push_str(output);
        unlocked.push('\n');
        Ok(())
    }

    fn flush(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

impl fmt::Display for OutputCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("_stdout_")
    }
}

// Runs an input program and returns the output as a String
pub fn compile_and_run(input: &str) -> String {
    let output = Rc::new(RefCell::new(String::new()));

    let mut koto = Koto::with_settings(
        KotoSettings::default()
            .with_stdout(OutputCapture {
                output: output.clone(),
            })
            .with_stderr(OutputCapture {
                output: output.clone(),
            }),
    );

    match koto.compile(input) {
        Ok(_) => match koto.run() {
            Ok(_) => std::mem::take(&mut output.borrow_mut()),
            Err(error) => format!("Runtime error: {error}"),
        },
        Err(error) => format!("Compilation error: {error}"),
    }
}
