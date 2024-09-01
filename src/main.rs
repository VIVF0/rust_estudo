use core::num;
use std::{string, vec};

//const n é armazenado em memoria, em tempo de compilação a variavel é substituida pelo valor passado
const PI:f32 = 3.14;
//variavel global
static mut VARIAVEL_GLOBAL:u8 = 1;
//Função main igual C
fn main() {
    escopo();
    sombra();

    let valor:i32 = soma(1,2);
    println!("{}", valor);

    let maior_idade:bool = verifica_idade(17);
    println!("{}", maior_idade);
    repeticao();

    let jogo = "LOL";

    //parecido com um Switch sendo o _ uma opção não escrita
    let resultado = match jogo {
        "LOL" => true,
        "CS" => false,
        "DOTA" => false,
        _ => false
    };

    println!("{}", resultado);

    ownership();

    patteern_matching();

    erros();
}

fn ownership() {
    let mut um_string = String::from("Vitor");
    //passando a dominancia da memoria:
    //let outra_string = rouba(um_string);

    //por referencia
    rouba(&mut um_string);

    println!("{}", um_string);
}

/* Por dominancia:
fn rouba(string: String) -> String {
    println!("{}", string);

    string
}*/

//referencia proximo ao ponteiro do C
fn rouba(string: &mut String){
    string.push_str(" freire");
    println!("{}", string);
}

fn patteern_matching(){
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouco",
            4..=10 => "Bastante",
            _ if x % 2 == 0 => "Boa quantidade",
            _ => "Muito"
        });
    }
}

fn erros() {
    match resultado() {
        Ok(s) => println!("String de sucesso: {}", s),
        Err(numero) => println!("Codigo de erro {}", numero)
    };
}

fn resultado() -> Result<String, u8> {
    Ok(String::from("Tudo deu certo!"))
    //Err(42)
}

fn repeticao(){
    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;
    println!("WHILE:");
    while contador < 10{
        contador += 1;

        //não seguir o laço e seguir para o proximo loop
        if contador == 5 {
            continue;
        }

        println!("{} X {} = {}", multiplicador, contador, multiplicador*contador);
    }

    contador = 0;
    println!("LOOP:");
    loop {
        //reponsavel por limitar o loop
        if contador == 10 {
            break;
        }
        println!("{} X {} = {}", multiplicador, contador, multiplicador*contador);
        contador += 1;
    }

    println!("FOR:");
    //for no rust é igual foreach, 1..11 gera um range de 1 até 10
    for i in 1..11 {
        println!("{} X {} = {}", multiplicador, i, multiplicador*i);
    }
}

fn verifica_idade(idade:i8) ->bool{
    let responsavel_autorizou:bool = true;
    let maior_idade: bool = idade >= 18;

    if maior_idade {
        println!("Pode entrar na balada!");
    } else if (idade > 16 && responsavel_autorizou) {
        println!("Pode entrar na balada!");
    } else {
        println!("Não pode entrar na balada!");
    }

    let condicao = if maior_idade {"Maior"} else {"Menor"};

    println!("{}", condicao);

    maior_idade
}

fn soma(a:i32, b:i32) -> i32{
    println!("{} + {} = {}", a, b , a+b);
    // return: (SEM ;)
    a + b
}

fn sombra(){
    let a = 123;

    // o codigo abaixo fica restrito aos dados de dentro das {}, podendo acessar coisas de fora das {} dentro da funcao sombra
    {
        let b = 456;
        println!("dentro: b={}", b);
        let a = 777;
        println!("dentro: a={}", a);
    }

    println!("fora a: {}", a);
    //println!("b fora: {}", b);
}

fn escopo(){
    //let usando para declarar variavel
    //i8 = inteiro de 8bits
    // oq vem dps de : é a declaração do tipo
    let variavel:u8 = 128;

    let variavel_sem_declaracao = 128;

    println!("tamanho: {}", std::mem::size_of_val(&variavel_sem_declaracao));
    // {} espaço onde sera impressso a variavel passada dps da virgula igual C
    println!("BLA BLA BLA = {}", variavel);

    let decimal:f32 = 2.5;

    println!("tamanho: {}", std::mem::size_of_val(&decimal));

    //let mut são variavel que são mutaveis, por padrao quando é colocado apenas let, a variavel não pode ter seu valor alterado.
    let mut booleana:bool = true;
    println!("tamanho: {}", std::mem::size_of_val(&booleana));
    
    let letra:char = 'C';

    println!("tamanho: {}", std::mem::size_of_val(&letra));

    //para usar variavel global mutaveis é preciso declarar que estou siente que isso não é seguro
    unsafe {
        println!("variavel global: {}", VARIAVEL_GLOBAL);
    }

    println!("PI: {}", PI);
}
