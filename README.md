# slr1aux
Ferramenta para gerar automaticamente o automato e a tabela SLR. É gerado também uma estrutura de dados contendo o não terminal e o número de símbolos de cada produção da gramática para tornar as reduções possíveis. A tabela é gerada tanto em formato MD quanto em código Rust.
## Execução
```
cargo run -- caminho/para/o/arquivo/da/gramatica.txt > saida.txt
```
## Formato da gramática
```
S' -> T
T -> F
T -> T * F
F -> id
F -> ( T )
```
A gramática aceita pelo programa é do tipo Gramática Aumentada e, por isso, estado inicial deve ser sempre S'.
## Exemplo de automato gerado (exemplo-slide.txt)
```

---------------------------------
I0:
  S' -> .T
  T -> .F
  T -> .T Mult F
  F -> .Id
  F -> .AbreP T FechaP

δ(I0, T) = I1
δ(I0, F) = I4
δ(I0, Id) = I5
δ(I0, AbreP) = I6

---------------------------------
I1:
  S' -> T.
  T -> T .Mult F

δ(I1, Mult) = I2

---------------------------------
I2:
  T -> T Mult .F
  F -> .Id
  F -> .AbreP T FechaP

δ(I2, F) = I3
δ(I2, Id) = I5
δ(I2, AbreP) = I6

---------------------------------
I3:
  T -> T Mult F.

---------------------------------
I4:
  T -> F.

---------------------------------
I5:
  F -> Id.

---------------------------------
I6:
  F -> AbreP .T FechaP
  T -> .F
  T -> .T Mult F
  F -> .Id
  F -> .AbreP T FechaP

δ(I6, T) = I7
δ(I6, F) = I4
δ(I6, Id) = I5
δ(I6, AbreP) = I6

---------------------------------
I7:
  F -> AbreP T .FechaP
  T -> T .Mult F

δ(I7, FechaP) = I8
δ(I7, Mult) = I2

---------------------------------
I8:
  F -> AbreP T FechaP.
```
## Exemplo de tabela MD gerada (exemplo-slide.txt)
| Estado | * | id | ( | ) | $ | T | F |
|---|---|---|---|---|---|---|---|
| I0 | ```erro``` | ```E5``` | ```E6``` | ```erro``` | ```erro``` | ```1``` | ```4``` | 
| I1 | ```E2``` | ```erro``` | ```erro``` | ```erro``` | ```ACEITAR``` |   |   | 
| I2 | ```erro``` | ```E5``` | ```E6``` | ```erro``` | ```erro``` |   | ```3``` | 
| I3 | ```R2``` | ```R2``` | ```R2``` | ```R2``` | ```R2``` |   |   | 
| I4 | ```R1``` | ```R1``` | ```R1``` | ```R1``` | ```R1``` |   |   | 
| I5 | ```R3``` | ```R3``` | ```R3``` | ```R3``` | ```R3``` |   |   | 
| I6 | ```erro``` | ```E5``` | ```E6``` | ```erro``` | ```erro``` | ```7``` | ```4``` | 
| I7 | ```E2``` | ```erro``` | ```erro``` | ```E8``` | ```erro``` |   |   | 
| I8 | ```R4``` | ```R4``` | ```R4``` | ```R4``` | ```R4``` |   |   | 
## Exemplo de estrutura com a contagem de símbolos das produções (exemplo-slide.txt)
Esse trecho de código será inserido no início na função `analisar`.
```
let producoes = vec![
    (NaoTerminais::S', 1 as usize),
    (NaoTerminais::T, 1 as usize),
    (NaoTerminais::T, 3 as usize),
    (NaoTerminais::F, 1 as usize),
    (NaoTerminais::F, 3 as usize),
];
```
OBS: S' será renomeado para SL.
## Exemplo de tabela em Rust gerada (exemplo-slide.txt)
Esse código será o conteúdo do match na função `obtem_acao` no liac.
```
            0 => {
                if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Empilha(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Empilha(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::T) = simbolo {
                    return Ok(Acoes::VaiPara(1));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            1 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Empilha(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Aceita);
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            2 => {
                if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Empilha(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Empilha(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(3));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            3 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            4 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            5 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            6 => {
                if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Empilha(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Empilha(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::T) = simbolo {
                    return Ok(Acoes::VaiPara(7));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            7 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Empilha(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Empilha(8));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            8 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            _ => {
                return Err(());
            },
```