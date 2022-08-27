use clap::Parser;

/// Este comentario será la descripción del programa
#[derive(Parser, Debug)]
// Y esto es lo que va a mostrar la ayuda (--help)
#[clap(author, version, about)]
struct Cli {
   /// Descripción del parámetro
   #[clap(short, long, value_parser, default_value = "Default")]
   mensaje: String,

   /// Otra descripción, con un valor por defecto
   #[clap(short, long, value_parser, default_value_t = 10)]
   numero: u32,
}

fn main() {
   let args = Cli::parse();

   println!("Mensaje: {}", args.mensaje);
   println!("Numero: {}", args.numero);
}
