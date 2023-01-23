use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ItemLR {
    producao: usize,
    posicao_do_ponto: usize,
}

#[derive(Debug, Clone)]
struct RegraDeProducao {
    nao_terminal: String,
    producao: Vec<String>,
}

#[derive(Debug, Clone)]
struct Gramatica {
    regras: Vec<RegraDeProducao>,
    nao_terminais: Vec<String>,
    terminais: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Transicao {
    posicao: usize,
    simbolo: String,
    producoes: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Estado {
    producoes_iniciais: Vec<usize>,
    posicao_do_ponto: usize,
    itens: Vec<ItemLR>,
    transicoes: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Automato {
    gramatica: Gramatica,
    estados: Vec<Estado>,
    transicoes: Vec<Transicao>,
}

fn main() {
    // obtém argumentos do terminal
    let argumentos: Vec<String> = env::args()
        .collect();

    // abre arquivo
    let mut arquivo = File::open(argumentos.get(1).unwrap())
        .unwrap();
    
    // variável para armazenar o conteúdo do arquivo
    let mut conteudo_arquivo = String::new();

    // lê o conteúdo do arquivo
    arquivo.read_to_string(&mut conteudo_arquivo)
        .unwrap();
    
    // separa conteúdo em linhas
    let linhas_arquivo: Vec<&str> = conteudo_arquivo
        .split("\n")
        .collect();

    // obtem gramática
    let regras = obtem_regras_de_producao(linhas_arquivo);
    let nao_terminais = obtem_nao_terminais(regras.to_owned());
    let gramatica = Gramatica {
        regras: regras.to_owned(),
        nao_terminais: nao_terminais.to_owned(),
        terminais: obtem_terminais(regras, nao_terminais),
    };

    // gera o autômato
    let mut automato = Automato::inicializa(gramatica);
    automato.analiza();
    automato.resultado();
}

fn obtem_regras_de_producao(linhas_arquivo: Vec<&str>) -> Vec<RegraDeProducao> {
    let mut regras_de_producao: Vec<RegraDeProducao> = Vec::new();

    // lê linha por linha para obter a gramática
    for linha in linhas_arquivo {
        let split_flecha: Vec<&str> = linha
            .split(" -> ")
            .collect();
        
        let split_espaco: Vec<&str> = split_flecha[1]
            .split(" ")
            .collect();
        
        let regra_de_producao: RegraDeProducao = RegraDeProducao {
            nao_terminal: split_flecha[0].to_string(),
            producao: split_espaco
                .iter()
                .map(|s| s.to_string())
                .collect(),
        };

        // armazena resultados nos vetores
        regras_de_producao.push(regra_de_producao);
    }

    return regras_de_producao;
}

fn obtem_nao_terminais(regras_de_producao: Vec<RegraDeProducao>) -> Vec<String> {
    let mut nao_terminais: Vec<String> = Vec::new();
    for regra_de_producao in regras_de_producao {
        if !nao_terminais.iter().any(|s| *s == regra_de_producao.nao_terminal) {
            nao_terminais.push(regra_de_producao.nao_terminal);
        }
    }
    return nao_terminais;
}

fn obtem_terminais(regras_de_producao: Vec<RegraDeProducao>, nao_terminais: Vec<String>) -> Vec<String> {
    let mut terminais: Vec<String> = Vec::new();
    for regra_de_producao in regras_de_producao {
        for simbolo in regra_de_producao.producao {
            if (!terminais.iter().any(|s| *s == simbolo)) && (!nao_terminais.iter().any(|s| *s == simbolo)){
                terminais.push(simbolo);
            }
        }
    }
    return terminais;
}

impl Automato {
    fn inicializa(gramatica: Gramatica) -> Self {
        Automato {
            gramatica: gramatica,
            estados: Vec::new(),
            transicoes: Vec::new(),
        }
    }

    fn converte_para_item(&mut self, producao: usize, posicao_do_ponto: usize) -> ItemLR {
        ItemLR {
            producao: producao,
            posicao_do_ponto: posicao_do_ponto,
        }
    }

    fn analiza(&mut self) {
        self.gera_estado(vec![0], 0);
    }

    fn gera_estado(&mut self, producoes_iniciais: Vec<usize>, posicao_do_ponto: usize) {
        let mut itens: Vec<ItemLR> = Vec::new();
        let mut transicoes: Vec<Transicao> = Vec::new();


        // adiciona as produções iniciais ao vetor de itens
        for producao in producoes_iniciais.to_owned() {
            itens.push(self.converte_para_item(producao, posicao_do_ponto));
        }

        // adiciona os desvios ao vetor de itens
        let mut contador: usize = 0;
        let mut tamanho_vetor_itens: usize = itens.len();
        while contador < tamanho_vetor_itens {
            let item = itens[contador].clone();
            if item.posicao_do_ponto < self.gramatica.regras[item.producao].producao.len() {
                let simbolo_marcado = self.gramatica.regras[item.producao].producao[item.posicao_do_ponto].to_string();
                if self.gramatica.nao_terminais.iter().any(|s| *s == simbolo_marcado) {
                    for i in 0..self.gramatica.regras.len() {
                        let producao = self.gramatica.regras[i].clone();
                        if producao.nao_terminal == simbolo_marcado {
                            let novo_item = ItemLR {
                                producao: i,
                                posicao_do_ponto: 0,
                            };
                            if !itens.iter().any(|nl| *nl == novo_item) {
                                itens.push(novo_item);
                            }
                        }
                    }
                }
                let transicao = Transicao {
                    simbolo: simbolo_marcado,
                    posicao: item.posicao_do_ponto,
                    producoes: vec![item.producao],
                };
                if !transicoes.iter().any(|v| *v == transicao.to_owned()) {
                    transicoes.push(transicao);
                }
            }
            contador += 1;
            tamanho_vetor_itens = itens.len();
        }
               
        // junta transições sobre o mesmo símbolo
        contador = 0;
        let mut outro_contador: usize;
        let mut tamanho_vetor_transicoes: usize = transicoes.len();
        while contador < tamanho_vetor_transicoes {
            outro_contador = contador + 1;
            while outro_contador < tamanho_vetor_transicoes {
                if transicoes[contador].simbolo == transicoes[outro_contador].simbolo {
                    let producao = transicoes[outro_contador].producoes[0];
                    transicoes.remove(outro_contador);
                    transicoes[contador].producoes.push(producao);
                }
                outro_contador += 1;
                tamanho_vetor_transicoes = transicoes.len();
            }
            contador += 1;
            tamanho_vetor_transicoes = transicoes.len();
        }


        //
        let mut transicoes_index: Vec<usize> = Vec::new();
        let transicoes_tamanho_anterior = self.transicoes.len();
        contador = 0;
        for i in 0..transicoes.len() {
            let transicao = transicoes[i].clone();
            if let Some(index) = self.transicoes.iter().enumerate().find(|(_, t)| t.simbolo == transicao.simbolo && t.posicao == transicao.posicao && t.producoes == transicao.producoes) {
                transicoes_index.push(index.0);
            } else {
                transicoes_index.push(self.transicoes.len());
                self.transicoes.push(transicoes[i].clone());
                contador += 1;
            }
        }

        // registra estado
        let estado = Estado {
            producoes_iniciais: producoes_iniciais,
            posicao_do_ponto: posicao_do_ponto,
            itens: itens,
            transicoes: transicoes_index.to_vec(),
        };
        self.estados.push(estado);

        println!("{:?}", self.estados);
        println!("{:?}\n", self.transicoes);

        // gera estados sobre novas transições
        for transicao in transicoes_index.to_vec() {
            if transicao >= transicoes_tamanho_anterior {
                self.gera_estado(self.transicoes[transicao].producoes.to_vec(), self.transicoes[transicao].posicao + 1);
            }
        }
    }

    fn obtem_destino(&self, transicao: Transicao) -> usize {
        let mut destino: usize = 0;
        for i in self.estados.to_vec() {
            if i.posicao_do_ponto == transicao.posicao + 1 && i.producoes_iniciais == transicao.producoes {
                break;
            }
            destino += 1;
        }
        return destino;
    }

    fn resultado(&self) {
        for i in 0..self.estados.len() {
            println!("\n---------------------------------");
            println!("I{}:", i);
            self.printa_itens(self.estados[i].itens.to_vec());
            if self.estados[i].transicoes.len() > 0 {
                print!("\n");
                self.printa_transicoes(i);
            }
        }
    }

    fn printa_itens(&self, itens: Vec<ItemLR>) {
        for item in itens {
            let mut i: usize = 0;
            let mut string: String = "  ".to_string() + &self.gramatica.regras[item.producao].nao_terminal.to_string() + " ->";
            for simbolo in self.gramatica.regras[item.producao].producao.to_owned() {
                if i == item.posicao_do_ponto {
                    string = string + " .";
                } else {
                    string = string + " ";
                }
                string = string + simbolo.as_ref();
                if i + 1 == item.posicao_do_ponto && i + 2 == self.gramatica.regras[item.producao].producao.len() + 1 {
                    string = string + ".";
                }
                i = i + 1;
            }
            println!("{}", string);
        }
    }

    fn printa_transicoes(&self, estado: usize) {
        for transicao in self.estados[estado].transicoes.to_vec() {
            println!("δ(I{}, {}) = I{}", estado, self.transicoes[transicao].simbolo, self.obtem_destino(self.transicoes[transicao].clone()));
        }
    }
}
