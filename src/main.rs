use std::env;
use std::fs::File;
use std::io::prelude::*;

use simple_matrix::Matrix;

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
struct Seguinte {
    nao_terminal: String,
    terminais: Vec<String>,
}

#[derive(Debug, Clone)]
struct Gramatica {
    regras: Vec<RegraDeProducao>,
    nao_terminais: Vec<String>,
    terminais: Vec<String>,
    seguintes: Vec<Seguinte>
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

    // abre arquivos
    let mut arquivo_gramatica = File::open(argumentos.get(1).unwrap())
        .unwrap();
    let mut arquivo_seguintes = File::open(argumentos.get(2).unwrap())
        .unwrap();
    
    // variáveis para armazenar os conteúdos dos arquivos
    let mut conteudo_arquivo_gramatica = String::new();
    let mut conteudo_arquivo_seguintes = String::new();

    // lê o conteúdo dos arquivos
    arquivo_gramatica.read_to_string(&mut conteudo_arquivo_gramatica)
        .unwrap();
    arquivo_seguintes.read_to_string(&mut conteudo_arquivo_seguintes)
        .unwrap();
    
    // separa conteúdo em linhas
    let linhas_arquivo_gramatica: Vec<&str> = conteudo_arquivo_gramatica
        .split("\n")
        .collect();
    let linhas_arquivo_seguintes: Vec<&str> = conteudo_arquivo_gramatica
        .split("\n")
        .collect();
    
    // obtem seguintes
    let seguintes = obtem_seguintes(linhas_arquivo_seguintes);

    // obtem gramática
    let regras = obtem_regras_de_producao(linhas_arquivo_gramatica);
    let nao_terminais = obtem_nao_terminais(regras.to_owned());
    let gramatica = Gramatica {
        regras: regras.to_owned(),
        nao_terminais: nao_terminais.to_owned(),
        terminais: obtem_terminais(regras, nao_terminais),
        seguintes: seguintes,
    };

    // gera o autômato
    let mut automato = Automato::inicializa(gramatica);
    automato.analiza();
    automato.resultado();
    automato.gera_tabela_md();
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

fn obtem_seguintes(linhas_arquivo: Vec<&str>) -> Vec<Seguinte> {
    let mut seguintes: Vec<Seguinte> = Vec::new();

    // lê linha por linha para obter os seguintes
    for linha in linhas_arquivo {
        let split_flecha: Vec<&str> = linha
            .split(" : ")
            .collect();
        if split_flecha.len() > 1 {
            let split_espaco: Vec<&str> = split_flecha[1]
                .split(" ")
                .collect();
            
            let seguinte = Seguinte {
                nao_terminal: split_flecha[0].to_string(),
                terminais: split_espaco
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            };

            // armazena resultados nos vetores
            seguintes.push(seguinte);
        } else {
            let seguinte = Seguinte {
                nao_terminal: split_flecha[0].to_string(),
                terminais: Vec::new(),
            };

            // armazena resultados nos vetores
            seguintes.push(seguinte);
        }
    }

    return seguintes;
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

        // registra transições no vetor
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

    fn cabecalho_md(&self) -> String {
        let mut string1: String = "| Estado ".to_string();
        let mut string2: String = "|---".to_string();
        for i in self.gramatica.terminais.to_owned() {
            string1 = format!("{}| {} ", string1, i);
            string2 = format!("{}|---", string2);
        }
        string1 = format!("{}| $ ", string1);
        string2 = format!("{}|---", string2);
        for i in self.gramatica.nao_terminais.to_owned() {
            if i != "S'" {
                string1 = format!("{}| {} ", string1, i);
                string2 = format!("{}|---", string2);
            }
        }
        return format!("{}|\n{}|\n", string1, string2);
    }

    fn gera_tabela_md(&self) {
        let tabela = self.gera_tabela();

        let mut string: String = self.cabecalho_md();
        let mut contador: usize = 0;

        for i in 0..self.estados.len() {
            string += format!("| I{} | ", contador).as_ref();
            for j in 0..(self.gramatica.terminais.len() + self.gramatica.nao_terminais.len()) {
                let celula = tabela.get(i, j).unwrap();
                if celula != " " {
                    string += format!("```{}``` | ", celula).as_ref();
                } else {
                    string += "  | ";
                }
            }
            string += "\n";
            contador += 1;
        }

        println!("{}", string);
    }

    fn gera_tabela(&self) -> Matrix<String> {
        let mut tabela: Matrix<String> = Matrix::new(
            self.estados.len(),
            self.gramatica.terminais.len() + self.gramatica.nao_terminais.len(),
        );

        for i in 0..self.estados.len() {
            let estado = self.estados[i].clone();

            // colunas dos terminais
            for j in 0..self.gramatica.terminais.len() {
                let terminal = self.gramatica.terminais[j].clone();
                if estado.transicoes.iter().any(|t| self.transicoes[*t].simbolo == terminal) {
                    if let Some(transicao) = self.transicoes
                        .iter()
                        .enumerate()
                        .find(|(_, t)| t.simbolo == terminal)
                    {
                        tabela.set(i, j, format!("E{}", self.obtem_destino(transicao.1.clone())));
                    } else {
                        tabela.set(i, j, "erro".to_string());
                    }
                } else {
                    // caso contrário, verifica se há um item LR em estado final sobre outros não terminais
                    // neste caso, redução
                    if let Some(itemlr) = estado.itens.iter().enumerate().find(|(_, i)|
                        (self.gramatica.regras[i.producao].producao.len() == i.posicao_do_ponto)
                    ) {
                        let producao = itemlr.1.clone().producao;
                        let simbolo = self.gramatica.regras[producao].nao_terminal.clone();
                        if let Some(transicao) = estado.transicoes.iter().enumerate().find(|(_, t)|
                            (self.transicoes[**t].simbolo == simbolo)
                        ) {
                            tabela.set(i, j, format!("R{}", self.obtem_destino(self.transicoes[*transicao.1].clone())));
                        } else {
                            tabela.set(i, j, "erro".to_string());
                        }
                    }
                    // erro para os demais casos
                    else {
                        tabela.set(i, self.gramatica.terminais.len(), "erro".to_string());
                    }
                }
            }
            
            // coluna do $
            // verifica se há um item LR em estado final sobre S'
            // neste caso, aceita
            if estado.itens.iter().any(|i| 
                (self.gramatica.regras[i.producao].nao_terminal == "S'") &&
                (self.gramatica.regras[i.producao].producao.len() == i.posicao_do_ponto)
            ) {
                tabela.set(i, self.gramatica.terminais.len(), "ACEITAR".to_string());
            }
            // caso contrário, verifica se há um item LR em estado final sobre outros não terminais
            // neste caso, redução
            else if let Some(itemlr) = estado.itens.iter().enumerate().find(|(_, i)|
                (self.gramatica.regras[i.producao].producao.len() == i.posicao_do_ponto)
            ) {
                let producao = itemlr.1.clone().producao;
                let simbolo = self.gramatica.regras[producao].nao_terminal.clone();
                if let Some(transicao) = estado.transicoes.iter().enumerate().find(|(_, t)|
                    (self.transicoes[**t].simbolo == simbolo)
                ) {
                    tabela.set(i, self.gramatica.terminais.len(), format!("R{}", self.obtem_destino(self.transicoes[*transicao.1].clone())));
                } else {
                    tabela.set(i, self.gramatica.terminais.len(), "erro".to_string());
                }
            }
            // erro para os demais casos
            else {
                tabela.set(i, self.gramatica.terminais.len(), "erro".to_string());
            }

            // colunas dos não terminais
            for j in 0..self.gramatica.nao_terminais.len() {
                let nao_terminal = self.gramatica.nao_terminais[j].clone();
                if nao_terminal != "S'" {
                    if let Some(transicao) = estado.transicoes
                        .iter()
                        .enumerate()
                        .find(|(_, t)| self.transicoes[**t].simbolo == nao_terminal)
                    {
                        println!("Aqui");
                        tabela.set(i, j + self.gramatica.terminais.len(), format!("{}", self.obtem_destino(self.transicoes[*transicao.1].clone())));
                    } else {
                        tabela.set(i, j + self.gramatica.terminais.len(), " ".to_string());
                    }
                }
            }
        }
        return tabela;
    }
}
