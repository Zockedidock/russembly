mod cpu;
mod lexer;

use crate::{cpu::get_global_output, cpu::main::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RussemblyWasm {
    cpu_json: String,
}
#[wasm_bindgen]
impl RussemblyWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RussemblyWasm {
        RussemblyWasm {
            cpu_json: String::from(""),
        }
    }
    pub fn run_rusm(&mut self, code: &str) -> String {
        let mut cpu = match CPU::new() {
            Ok(cpu) => cpu,
            Err(_) => {
                //log(&format!("{}", err));
                return "".to_string();
            }
        };
        if let Some(lines) = cpu.load_string(code) {
            cpu.run_lines(lines);
        }
        self.cpu_json = cpu.get_json();
        return get_global_output();
    }
    pub fn get_cpu_json(self) -> String {
        self.cpu_json
    }
}
