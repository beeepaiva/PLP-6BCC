use structopt::StructOpt;

// 1 - Definindo uma estrutura da linha de comando
#[derive(StructOpt)]
struct Cli{
  padrao: String,
  #[structopt(parse(from_os_str))]
  arquivo: std::path::PathBuf,
}

//Por padrão o o software apenas lê o arquivo texto. Porém, pode ter a opção –l
//que só exibe a linha e ou –w que só exibe a quantidade de palavras

//No caso desse exercicio, eu poderia declarar os contadores como i8, por saber que não possui quantidade para estourar o limite dentro do arquivo de teste,
//mas pensando em outros textos, escolhi utilizar o i32 por trazer uma range maior de numeros para caber o quantidade de palavras e/ou linhas de um aquivo maior

fn main() {
  // 2 - Padrões de entrada
  let padrao = std::env::args().nth(1).expect("no pattern given");
  let caminho = std::env::args().nth(2).expect("no path given");
  let args = Cli {
    padrao: padrao,
    arquivo: std::path::PathBuf::from(caminho),
   };
  let args = Cli::from_args();

  let content = std::fs::read_to_string(&args.arquivo)
    .expect("could not read file");

  let comando = &args.padrao;
  let mut contadorLinha : i32 = 0;
  let mut contadorPalavra : i32 = 0;

  for line in content.lines() {

  	contadorLinha += 1;

  	if line.contains(&args.padrao) {
      println!("{}", line);
    }

  	if comando == "-w" {
  		let palavras : Vec<&str> = line.split(' ').collect();
  		for palavra in palavras { 
  			contadorPalavra += 1; 
  		}
  		println!("{}", contadorPalavra);	
  	}

    if comando == "-l" {
      println!("{}", contadorLinha);
    }


  }

}
