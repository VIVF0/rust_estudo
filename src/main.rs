//const n é armazenado em memoria, em tempo de compilação a variavel é substituida pelo valor passado
const PI:f32 = 3.14;
//variavel global
static mut VARIAVEL_GLOBAL:u8 = 1;
//Função main igual C
fn main() {
    escopo();
    sombra();
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
