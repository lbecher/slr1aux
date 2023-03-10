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
struct Gramatica {
    regras: Vec<RegraDeProducao>,
    nao_terminais: Vec<String>,
    terminais: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Transicao {
    simbolo: String,
    itens: Vec<ItemLR>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Estado {
    itens_iniciais: Vec<ItemLR>,
    itens: Vec<ItemLR>,
    transicoes: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Automato {
    gramatica: Gramatica,
    estados: Vec<Estado>,
    transicoes: Vec<Transicao>,
    tabela: Matrix<String>,
}

fn main() {
    // obtém argumentos do terminal
    let argumentos: Vec<String> = env::args()
        .collect();

    // abre arquivos
    let mut arquivo_gramatica = File::open(argumentos.get(1).unwrap())
        .unwrap();
    
    // variáveis para armazenar os conteúdos dos arquivos
    let mut conteudo_arquivo_gramatica = String::new();

    // lê o conteúdo dos arquivos
    arquivo_gramatica.read_to_string(&mut conteudo_arquivo_gramatica)
        .unwrap();
    
    // separa conteúdo em linhas
    let linhas_arquivo_gramatica: Vec<&str> = conteudo_arquivo_gramatica
        .split("\n")
        .collect();

    // obtem gramática
    let regras = obtem_regras_de_producao(linhas_arquivo_gramatica);
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
    automato.gera_tabela();
    automato.gera_tabela_md();
    automato.gera_tabela_rust();
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
            tabela: Matrix::new(1,1),
        }
    }

    fn analiza(&mut self) {
        let item_inicial = ItemLR {
            producao: 0,
            posicao_do_ponto: 0,
        };
        self.gera_estado(vec![item_inicial]);
    }

    fn gera_estado(&mut self, itens_iniciais: Vec<ItemLR>) {
        let mut itens: Vec<ItemLR> = Vec::new();
        let mut transicoes: Vec<Transicao> = Vec::new();

        // adiciona as produções iniciais ao vetor de itens
        for item in itens_iniciais.to_owned() {
            itens.push(item.clone());
        }

        // adiciona os desvios ao vetor de itens
        let mut contador: usize = 0;
        let mut tamanho_vetor_itens: usize = itens.len();
        // se tamanho diferente, mais desvios devem ser analisados
        while contador < tamanho_vetor_itens {
            let item = itens[contador].clone();
            // se não um item final
            if item.posicao_do_ponto < self.gramatica.regras[item.producao].producao.len() {
                let simbolo_marcado = self.gramatica.regras[item.producao].producao[item.posicao_do_ponto].to_string();
                // se o símbolo marcado for um não terminal
                if self.gramatica.nao_terminais.iter().any(|s| *s == simbolo_marcado) {
                    // percorre regras de produção
                    for i in 0..self.gramatica.regras.len() {
                        let producao = self.gramatica.regras[i].clone();
                        // encontra produção com símbolo marcado pelo ponto
                        if producao.nao_terminal == simbolo_marcado {
                            let novo_item = ItemLR {
                                producao: i,
                                posicao_do_ponto: 0,
                            };
                            // se o item ainda não foi adicionado, adiciona
                            if !itens.iter().any(|nl| *nl == novo_item) {
                                itens.push(novo_item);
                            }
                        }
                    }
                }
                let transicao = Transicao {
                    simbolo: simbolo_marcado,
                    itens: vec![item],
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
                    let item = transicoes[outro_contador].itens[0].clone();
                    transicoes.remove(outro_contador);
                    transicoes[contador].itens.push(item);
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
            if let Some(index) = self.transicoes.iter().enumerate().find(|(_, t)| 
                (**t == transicao)
            ) {
                transicoes_index.push(index.0);
            } else {
                transicoes_index.push(self.transicoes.len());
                self.transicoes.push(transicoes[i].clone());
                contador += 1;
            }
        }

        // registra estado
        let estado = Estado {
            itens_iniciais: itens_iniciais,
            itens: itens,
            transicoes: transicoes_index.to_vec(),
        };
        self.estados.push(estado);

        // gera estados sobre novas transições
        for transicao in transicoes_index.to_owned() {
            if transicao >= transicoes_tamanho_anterior {
                let mut itens_da_trasicao: Vec<ItemLR> = Vec::new();
                for item in self.transicoes[transicao].itens.to_owned() {
                    itens_da_trasicao.push(item);
                    let index = itens_da_trasicao.len();
                    itens_da_trasicao[index - 1].posicao_do_ponto += 1; 
                }
                self.gera_estado(itens_da_trasicao);
            }
        }
    }

    fn obtem_estado(&self, transicao: Transicao) -> usize {
        let mut destino: usize = 0;

        for i in self.estados.to_vec() {
            let mut itens = transicao.itens.to_vec();

            for j in 0..itens.len() {
                itens[j].posicao_do_ponto += 1;
            }

            if i.itens_iniciais == itens {
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
            println!("δ(I{}, {}) = I{}", 
                estado,
                self.transicoes[transicao].simbolo,
                self.obtem_estado(self.transicoes[transicao].clone())
            );
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
        let tabela = self.tabela.to_owned();

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

    fn gera_tabela_rust(&self) {
        let tabela = self.tabela.to_owned();

        let mut string: String = String::new();
        let mut contador: usize = 0;

        string += "let producoes = vec![\n";
        for i in self.gramatica.regras.to_owned() {
            string += format!("    (NaoTerminais::{}, {} as usize),\n", i.nao_terminal, i.producao.len()).as_ref();
        }
        string += "];\n";

        for i in 0..self.estados.len() {
            string += format!("            {} => {{\n", contador).as_ref();
            let mut condicoes: usize = 0;
            for j in 0..(self.gramatica.terminais.len() + self.gramatica.nao_terminais.len()) {
                let mut celula = tabela.get(i, j).unwrap().clone();
                if j < self.gramatica.terminais.len() {
                    if celula != "erro" {
                        if condicoes != 0 {
                            string += "                } else if ";
                        } else {
                            string += "                if ";
                        }
                        string += format!("let ElementosDaPilha::Tokens(Tokens::{}) = simbolo {{\n", self.gramatica.terminais[j]).as_ref();
                        if celula.contains("I") {
                            celula.remove(0);
                            string += format!("                    return Ok(Acoes::Empilha({}));\n", celula).as_ref();
                        } else {
                            celula.remove(0);
                            string += format!("                    return Ok(Acoes::Reduz({}));\n", celula).as_ref();
                        }
                        condicoes += 1;
                    }
                } else if j == self.gramatica.terminais.len() {
                    if celula != "erro" {
                        if condicoes != 0 {
                            string += "                } else if ";
                        } else {
                            string += "                if ";
                        }
                        string += "let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {\n";
                        if celula.contains("ACEITAR") {
                            string += "                    return Ok(Acoes::Aceita);\n";
                        } else {
                            celula.remove(0);
                            string += format!("                    return Ok(Acoes::Reduz({}));\n", celula).as_ref();
                        }
                        condicoes += 1;
                    }
                } else {
                    if celula != " " {
                        if condicoes != 0 {
                            string += "                } else if ";
                        } else {
                            string += "                if ";
                        }
                        string += format!("let ElementosDaPilha::NaoTerminais(NaoTerminais::{}) = simbolo {{\n", self.gramatica.nao_terminais[j - self.gramatica.terminais.len()]).as_ref();
                        string += format!("                    return Ok(Acoes::VaiPara({}));\n", celula).as_ref();
                        condicoes += 1;
                    }
                }
            }
            string += "                } else {\n                    return Ok(Acoes::Erro);\n                }\n            },\n";
            contador += 1;
        }

        string += "            _ => {\n                return Err(());\n            },\n";

        println!("{}", string);
    }

    fn gera_tabela(&mut self) {
        // define a tabela
        let mut tabela: Matrix<String> = Matrix::new(
            self.estados.len(),
            self.gramatica.terminais.len() + self.gramatica.nao_terminais.len(),
        );

        // estados
        for i in 0..self.estados.len() {
            let estado = self.estados[i].clone();

            // colunas dos terminais
            for j in 0..self.gramatica.terminais.len() {
                let terminal = self.gramatica.terminais[j].clone();

                tabela.set(
                    i, 
                    j, 
                    self.determina_acao_terminal(estado.clone(), terminal)
                );
            }
            
            // coluna do $
            tabela.set(
                i, 
                self.gramatica.terminais.len(), 
                self.determina_acao_final(estado.clone())
            );

            // colunas dos não terminais
            for j in 1..self.gramatica.nao_terminais.len() {
                let nao_terminal = self.gramatica.nao_terminais[j].clone();
                
                tabela.set(
                    i, 
                    j + self.gramatica.terminais.len(), 
                    self.determina_acao_nao_terminal(estado.clone(), nao_terminal)
                );
            }
        }
        self.tabela = tabela;
    }

    fn determina_acao_terminal(&self, estado: Estado, terminal: String) -> String {
        let mut acao = "erro".to_string();

        if let Some(transicao) = estado.transicoes
            .iter()
            .enumerate()
            .find(|(_, t)| 
                (self.transicoes[**t].simbolo == terminal)
        ) {
            acao = format!("I{}", self.obtem_estado(self.transicoes[*transicao.1].clone()));
        } else {
            // caso contrário, verifica se há um item LR em estado final sobre não terminais
            // neste caso, redução
            if let Some(itemlr) = estado.itens
                .iter()
                .enumerate()
                .find(|(_, i)|
                    (self.gramatica.regras[i.producao].producao.len() == i.posicao_do_ponto)
            ) {
                let producao = self.gramatica.regras[itemlr.1.producao].producao.to_vec();
                let regra = self.gramatica.regras.iter().enumerate().find(|(_, r)|
                    r.producao == producao
                ).unwrap().0;
                if regra > 0 {
                    acao = format!("R{}", regra);
                }
            }
        }

        return acao;
    }

    fn determina_acao_final(&self, estado: Estado) -> String {
        let mut acao = "erro".to_string();

        // verifica se há um item LR em estado final sobre S'
        // neste caso, aceita
        if estado.itens.iter().any(|i| 
            (self.gramatica.regras[i.producao].nao_terminal == "S'") &&
            (self.gramatica.regras[i.producao].producao.len() == i.posicao_do_ponto)
        ) {
            acao = "ACEITAR".to_string();
        }
        // caso contrário, verifica se há um item LR em estado final sobre outros não terminais
        // que não o S'. neste caso, redução
        else if let Some(itemlr) = estado.itens
            .iter()
            .enumerate()
            .find(|(_, i)|
                (self.gramatica.regras[i.producao].producao.len() == i.posicao_do_ponto)
        ) {
            let producao_tmp = self.gramatica.regras[itemlr.1.producao].producao.to_vec();
            let regra = self.gramatica.regras.iter().enumerate().find(|(_, r)|
                r.producao == producao_tmp
            ).unwrap().0;
            if regra > 0 {
                acao = format!("R{}", regra);
            }
        }

        return acao;
    }

    fn determina_acao_nao_terminal(&self, estado: Estado, nao_terminal: String) -> String {
        let mut acao = " ".to_string();

        if nao_terminal != "S'" {
            if let Some(transicao) = estado.transicoes
                .iter()
                .enumerate()
                .find(|(_, t)| 
                    (self.transicoes[**t].simbolo == nao_terminal)
            ) {
                acao = format!("{}", self.obtem_estado(self.transicoes[*transicao.1].clone()));
            }
        }

        return acao;
    }
}
