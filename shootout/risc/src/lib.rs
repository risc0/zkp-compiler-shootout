pub use methods_fib::{
    FIB_FIFTY_ID, FIB_FIFTY_PATH, FIB_ID, FIB_NINETY_TWO_ID, FIB_NINETY_TWO_PATH, FIB_PATH,
    SUDOKU_ID, SUDOKU_PATH,
};
use risc0_zkvm::{prove::Prover, receipt::Receipt};
use std::fs;

#[derive(Clone)]
pub struct Risc {
    pub method_id: &'static [u8],
    pub method_path: &'static str,
    pub input: Vec<u32>,
    pub name: String,
}

impl zero_knowledge::ZeroKnowledge for Risc {
    type C = Prover<'static>;
    type R = Receipt;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn compile(&self) -> Self::C {
        let file_contents = fs::read(self.method_path).unwrap();
        let mut prover = Prover::new(&file_contents, &self.method_id).unwrap();
        prover.add_input_u32_slice(&self.input);
        prover
    }

    fn prove(&self, setup: &mut Self::C) -> Self::R {
        setup.run().unwrap()
    }

    fn verify(&self, receipt: Self::R, _setup: &Self::C) {
        receipt.verify(self.method_id).unwrap()
    }
}

pub fn fib(input: u32) -> Risc {
    Risc {
        method_id: FIB_ID,
        method_path: FIB_PATH,
        input: vec![input],
        name: format!("Risc0: iter-{}", input),
    }
}
