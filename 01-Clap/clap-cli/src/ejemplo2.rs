use clap::{Subcommand, Parser};

/// En este ejemplo, vamos a implementar subcomados (crear y eliminar)
/// 
/// $ ejemplo2 crear -i 1 -m Test
/// $ ejemplo2 eleminar -i 1

#[derive(Subcommand, Debug)]
enum SubComando {

    /// Crear de un elemento
    Crear {
        #[clap(short, long, value_parser)]
        id: u32,

        #[clap(short, long, value_parser)]
        nombre: String,
    },

    /// Eliminar un elemento
    Eliminar {
        #[clap(short, long, value_parser)]
        id: u32,
    }
}

#[derive(Parser)]
struct Cli {

    #[clap(subcommand)]
    subcomando: SubComando,
}


fn main() {
    let cli = Cli::parse();
    println!("SubCommando: {:?}", cli.subcomando)
}