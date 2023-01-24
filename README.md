# slr1aux
Ferramenta para gerar automaticamente o automato SLR(1).
## Execução
```
cargo run -- caminho/para/o/arquivo/da/gramatica.txt caminho/para/o/arquivo/do/follow.txt
```
## Formato da gramática
```
S' -> T
T -> F
T -> T * F
F -> id
F -> ( T )
```
## Formato do follow
```
S' : 
T : * )
F : * )
```
OBS: Quando um follow não possui terminais, então deve-se adicionar um espaço após o ```:```.
## Exemplo de tabela gerada

| Estado | * | id | ( | ) | $ | S | F |
|---|---|---|---|---|---|---|---|
| I0 |   | ```E5``` | ```E6``` |   | ```erro``` |   |   | 
| I1 | ```E2``` |   |   |   | ```erro``` |   |   | 
| I2 |   | ```E5``` | ```E6``` |   | ```erro``` |   |   | 
| I3 |   |   |   |   | ```R1``` |   |   | 
| I4 |   |   |   |   | ```R1``` |   |   | 
| I5 |   |   |   |   | ```ACEITAR``` |   |   | 
| I6 |   | ```E5``` | ```E6``` |   | ```erro``` |   |   | 
| I7 |   | ```E5``` | ```E6``` | ```E8``` | ```erro``` |   |   | 
| I8 |   |   |   |   | ```ACEITAR``` |   |   | 