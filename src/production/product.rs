use crate::production::product_input::ProductInput;
use crate::production::product_output::ProductOutput;

pub struct Product {
    pub name: String,
    pub inputs: Vec<ProductInput>,
    // pub maker: // TODO: made in installation, spacecraft, settlement
    pub outputs: Vec<ProductOutput>,
}
