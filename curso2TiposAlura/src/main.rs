fn main() {
    //elemento de 4 espaços com o valor de 6.5
    let notas: [f32; 4] = [6.5;4];
    //tamanho adaptavel para guardar um endereco de memoria:
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    println!(
        "Nota1: {}, Nota2: {}, Nota3: {}, Nota4: {}",
        notas[0],
        notas[1],
        notas[2],
        notas[3]
    );

    for nota in notas {
        println!("Nota: {}", nota)
    }

    for i in 0..notas.len(){
        println!("Nota {} = {}", i, notas[i])
    }

    matriz();

    println!("Eh fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Domingo));

    cores();

    conteudo_opcinal();

    vectores();

    conta_corrente();
}

//Classe:
struct Conta {
    titular: Titular,
    saldo: f64
}

// Metodo de classe:
impl Conta {
    fn sacar(&mut self, valor: f64){
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

fn conta_corrente(){
    let mut conta: Conta = Conta {
        titular: Titular{nome: String::from("Vitor"), sobrenome: String::from("Freire")},
        saldo: 100.0
    };

    conta.sacar(20.5);

    println!("Titular: {}, saldo: {}", conta.titular.nome, conta.saldo);
}

fn vectores() {
    //let mut notas: Vec<u8> = Vec::new();
    //let mut notas: Vec<u8> = vec![10,85];
    //definir tamanho do array em sua inicialização
    let mut notas: Vec<u8> = Vec::with_capacity(4);
    println!("Capacidade array: {}", notas.capacity());
    notas.push(10);
    notas.push(8);

    println!("Capacidade 2 array: {}", notas.capacity());

    println!("{:?}",notas);

    println!("Nota 1: {}", notas[0]);

    println!("Nota 7: {}", match notas.get(7) {
        Some(n) => *n,
        None => 0
    });

    // Retorna o ultimo valor removendo ele:
    /*while let Some(nota) = notas.pop() {
        println!("Valor removido: {}", nota);
    }*/

    // Passando a referencia do array para ser percorrido no for:
    for nota in &notas {
        println!("Nota: {}", nota);
    }

    println!("Notas: {:?}", notas)
}

//sinalizando ao compilador que estou siente que o codigo abaixo nao esta sendo usado
#[allow(dead_code)]
enum Color {
    Red, Green, Blue, RgbColor(u8, u8, u8), CymkColor{cyan:u8, magenta: u8, yellow:u8, black:u8}
}

fn cores(){
    let cor = Color::CymkColor{cyan: 100, magenta: 50, yellow:70, black:255};

    println!("Cor: {}", match cor {
        Color::Red => "Vermelho",
        Color::Blue => "Azul",
        Color::Green => "Verde", 
        Color::RgbColor(0, 0, 0) | Color::CymkColor{cyan: _, magenta: _, yellow: _, black: 255} => "Black",
        Color::RgbColor(_, _, _) => "Desconhecido",
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "Cymk Desconhecido"
    });
}

fn matriz(){
    let matriz = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item: {}", item);
        }
    }
}

//Criação de tipos de dados:
enum  DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

fn conteudo_opcinal() {
    let conteudo_arquivo = ler_arquivo(String::from("jgkjbk"));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    }

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza que existe o valor");
    }

    /*while let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza que existe o valor");
    }*/
}
//option pode retornar o tipo que foi passado ou vazio
fn ler_arquivo(caminho_arquivo: String) -> Option<String>{
    //Some(String::from("Conteudo do arquivo"))
    None
}   